//! Common ATA controller functionality.

pub mod s10;
pub mod s11;
pub mod s5;
pub mod s8;
pub mod s9;

use std::fmt::Display;

use log::{debug, info};

use crate::protocol::{
    Transfer,
    ata::{
        self, SECTOR_SIZE,
        command::{Command, CommandRegisters},
        identify,
        log::phy_event_counters::CounterId,
    },
};

/// Phison ATA drive.
trait Drive: Display {
    /// Get ATA drive.
    fn ata(&self) -> &ata::Drive;
}

impl dyn Drive + '_ {
    /// Validate identify device result matches supported drive.
    fn validate_identify(identify: &identify::IdentifyDevice, drive: Option<&Self>) -> bool {
        const DLMC_MIN_BLOCKS: u16 = 1;

        let is_ssd = identify.rotation_rate == Some(identify::RotationRate::NonRotating);
        let has_smart = identify.features_supported.smart == Some(true)
            && identify.features_enabled.smart.is_some();
        let has_gpl = identify.features_supported.gpl == Some(true)
            && identify.features_enabled.gpl == Some(true);
        let id_has_phy_event_counters = identify
            .sata_capabilities
            .is_some_and(|x| x.phy_event_counters);
        let dlmc_min_blocks_match = matches!(identify.dlmc_min_blocks, Some(DLMC_MIN_BLOCKS));
        let valid =
            is_ssd && has_smart && has_gpl && id_has_phy_event_counters && dlmc_min_blocks_match;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (SSD: {is_ssd}, SMART: {has_smart}, GPL: \
                 {has_gpl}, PHY event counters: {id_has_phy_event_counters}, DLMC min blocks: \
                 {dlmc_min_blocks_match})"
            );
        }

        valid
    }

    /// Validate drive is supported.
    fn validate(&self) -> Result<bool, ata::Error> {
        const SMART_REVISION: u16 = 16;
        const COMPREHENSIVE_SMART_ERROR_PAGES: u16 = 51;
        const PHY_EVENT_COUNTER_IDS: &[CounterId] = &[
            CounterId::IcrcError,
            CounterId::RErrDataD2H,
            CounterId::RErrDataH2D,
            CounterId::RErrNondataD2H,
            CounterId::RErrNondataH2D,
            CounterId::NondataFisRetries,
            CounterId::PhyrdyToPhynrdy,
            CounterId::ComresetFis,
            CounterId::RErrDataH2DCrc,
            CounterId::RErrDataH2DNonCrc,
            CounterId::RErrNondataH2DCrc,
            CounterId::RErrNondataH2DNonCrc,
        ];

        let ata_drive = self.ata();

        // Check identify device fields
        let identify = ata_drive.identify_device()?;
        if !Self::validate_identify(&identify, Some(self)) {
            return Ok(false);
        }

        // Check GPL directory
        let gpl_directory = ata_drive.gpl_directory()?;
        let gpl_dir_phy_event = gpl_directory[ata::log::Log::PhyEventCounters] == 1;
        debug!(
            "[{self}] Validate GPL directory {}: {gpl_dir_phy_event}",
            ata::log::Log::PhyEventCounters,
        );
        if !gpl_dir_phy_event {
            return Ok(false);
        }

        // Check PHY event counters log
        let phy_event_counters = ata_drive.gpl_phy_event_counters()?;
        let phy_event_counters_match = phy_event_counters
            .iter()
            .map(|x| x.id)
            .eq(PHY_EVENT_COUNTER_IDS.iter().copied());
        debug!("[{self}] Validate GPL PHY event counters: {phy_event_counters_match}");
        if !phy_event_counters_match {
            return Ok(false);
        }

        // Ensure SMART is enabled for following checks
        ata_drive.smart_enable()?;

        // Check SMART data
        let smart = ata_drive.smart_read_data()?;
        // This is marked vendor-specific here and in ACS-3, but it's the old SMART
        // revision number
        let smart_revision =
            u16::from_le_bytes([smart.vendor_specific_1[0], smart.vendor_specific_1[1]]);
        let smart_revision_match = smart_revision == SMART_REVISION;
        debug!("[{self}] Validate SMART revision: {smart_revision_match}");
        if !smart_revision_match {
            return Ok(false);
        }

        // Check SMART log directory
        let smart_log_directory: ata::log::Directory = ata_drive.smart_log_directory()?;
        let smart_log_dir_comprehensive_smart = smart_log_directory
            [ata::log::Log::ComprehensiveSmartError]
            == COMPREHENSIVE_SMART_ERROR_PAGES;
        debug!(
            "[{self}] Validate SMART log directory {}: {smart_log_dir_comprehensive_smart}",
            ata::log::Log::ComprehensiveSmartError
        );
        if !smart_log_dir_comprehensive_smart {
            return Ok(false);
        }

        debug!("[{self}] Validated Phison ATA");
        Ok(true)
    }

    /// Execute VUC operation Phison calls 'set AP key', preparing for following
    /// transfer command.
    fn vuc_set_ap_key(&self) -> Result<(), ata::Error> {
        const COUNT: u16 = 0x6F;
        const LBA: u64 = 0xFA_EF_FE;
        const DEVICE: u8 = 0xAF;
        const COMMAND: Command = Command::VendorSpecific(0xE0);

        let registers = CommandRegisters {
            count: COUNT,
            lba: LBA,
            device: DEVICE,
            command: COMMAND,
            ..Default::default()
        };

        debug!("[{self}] Executing VUC set AP key");
        self.ata()
            .command(registers, Transfer::None, false, false, None)?;

        Ok(())
    }

    /// Execute VUC.
    fn vuc(&self, mut transfer: Transfer, feature: u8, lba: u32) -> Result<(), ata::Error> {
        const COMMAND_READ: Command = Command::VendorSpecific(0x21);
        const COMMAND_WRITE: Command = Command::VendorSpecific(0x31);
        const DEVICE: u8 = 0xA0;

        let command;
        match transfer {
            Transfer::None => {
                // No-data VUCs use write with single sector
                transfer = Transfer::Write(&[0u8; SECTOR_SIZE]);
                command = COMMAND_WRITE;
            },
            Transfer::Read(_) => command = COMMAND_READ,
            Transfer::Write(_) => command = COMMAND_WRITE,
        }

        let command_registers = CommandRegisters {
            feature: feature.into(),
            lba: lba.into(),
            device: DEVICE,
            command,
            ..Default::default()
        };

        self.vuc_set_ap_key()?;

        let log_transfer = transfer.to_string();
        debug!("[{self}] Executing VUC: {feature} (transfer: {log_transfer}, lba: {lba:#x})");

        let result_registers =
            self.ata()
                .command(command_registers, transfer, false, false, None)?;

        info!(
            "[{self}] Executed VUC: {feature} (transfer: {log_transfer}, lba: {lba:#x}, result \
             registers: {})",
            match result_registers {
                Some(x) => x.to_string(),
                None => "N/A".into(),
            }
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn validate_identify() {
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::corsair_nova2::IDENTIFY,      // S5
            test_data::kingston_ssdnow100::IDENTIFY, // S8
            test_data::patriot_blaze::IDENTIFY,      // S9
            test_data::ocz_trion150::IDENTIFY,       // S10
            test_data::kingston_a400::IDENTIFY,      // S11
            test_data::kingston_dc500r::IDENTIFY,    // S12
        ];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::adata_isss316::IDENTIFY,
            test_data::samsung_840::IDENTIFY,
            test_data::samsung_ss410::IDENTIFY,
        ];

        for &data in DATA_VALID {
            let identify = identify::IdentifyDevice::try_from(data).unwrap();
            assert!(<dyn Drive>::validate_identify(&identify, None));
        }

        for &data in DATA_INVALID {
            let identify = identify::IdentifyDevice::try_from(data).unwrap();
            assert!(!<dyn Drive>::validate_identify(&identify, None));
        }
    }
}
