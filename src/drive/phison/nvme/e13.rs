//! E13 controller.

mod info_block;
mod system_info;

use log::debug;

use super::VucOperation;
use crate::{
    cpu::{VECTOR_TABLE_SIZE, arm32},
    drive::{self, phison::VucLockState},
    protocol::{
        Transfer,
        nvme::{self, identify},
    },
};

/// Display name of drive type.
const DISPLAY_NAME: &str = "Phison E13";
/// Memory address of ARM exception vector table.
const EXCEPTION_VECTOR_TABLE_ADDRESS: u32 = 0;

/// E13 error.
#[derive(Debug)]
enum Error {
    /// NVMe error.
    Nvme(nvme::Error),
    /// Invalid system info data.
    InvalidSystemInfo,
    /// Invalid info block data.
    InvalidInfoBlock,
    /// Invalid exception vector table data in memory.
    InvalidExceptionVectorTable,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Nvme(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nvme(_) => write!(f, "NVMe error"),
            Self::InvalidSystemInfo => write!(f, "invalid system info"),
            Self::InvalidInfoBlock => write!(f, "invalid info block"),
            Self::InvalidExceptionVectorTable => write!(f, "invalid ARM exception vector table"),
        }
    }
}

impl drive::VendorError for Error {
    fn name(&self) -> &str {
        DISPLAY_NAME
    }
}

impl From<nvme::Error> for Error {
    fn from(value: nvme::Error) -> Self {
        Self::Nvme(value)
    }
}

impl From<Error> for drive::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Nvme(x) => x.into(),
            x => Self::Vendor(Box::new(x)),
        }
    }
}

/// Drive interface.
#[derive(Clone, Copy, Debug)]
struct Drive<'a> {
    /// NVMe drive.
    nvme: &'a nvme::Drive,
    /// Hosin HG2283 drive.
    hosin: bool,
}

impl<'a> Drive<'a> {
    /// Validate identify controller result matches supported drive.
    fn validate_identify(identify: &identify::controller::Identify, drive: Option<Self>) -> bool {
        const VERSION: identify::controller::Version = identify::controller::Version {
            major: 1,
            minor: 3,
            tertiary: 0,
        };
        const FIRMWARE_UPDATES: identify::controller::FirmwareUpdates =
            identify::controller::FirmwareUpdates {
                first_slot_read_only: false,
                slots: 1,
                activation_without_reset: true,
                multiple_update_detection: false,
            };
        const FW_UPDATE_GRANULARITY: Option<identify::controller::FirmwareUpdateGranularity> = Some(
            identify::controller::FirmwareUpdateGranularity::Size(16_384),
        );
        const SANITIZE: identify::controller::SanitizeCapabilities =
            identify::controller::SanitizeCapabilities {
                crypto_erase: false,
                block_erase: true,
                overwrite: true,
                verification: false,
                namespace_verification: false,
                purge_reporting: false,
                no_deallocate_inhibited: false,
                no_deallocate_modifies: None,
            };

        let version_match = identify.version == VERSION;
        let fw_updates_match = identify.firmware_updates == FIRMWARE_UPDATES;
        let fw_update_granularity_match = identify.fw_update_granularity == FW_UPDATE_GRANULARITY;
        let vsc_format_match = identify.admin_vsc_format && identify.nvm_vsc_format;
        let sanitize_match = identify.sanitize == SANITIZE;
        let valid = version_match
            && fw_updates_match
            && fw_update_granularity_match
            && vsc_format_match
            && sanitize_match;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (version: {version_match}, firmware \
                 updates: {fw_updates_match}, firmware update granularity: \
                 {fw_update_granularity_match}, VSC format: {vsc_format_match}, sanitize: \
                 {sanitize_match})"
            );
        }

        valid
    }

    /// Validate command effects log.
    fn validate_cmd_effects(
        cmd_effects: &nvme::log::command_effects::CommandEffects,
        drive: Option<Self>,
    ) -> bool {
        const EFFECTS_LBA_CHANGE_FALSE: nvme::log::command_effects::Effects =
            nvme::log::command_effects::Effects {
                logical_block_change: false,
                namespace_capability_change: false,
                namespace_inventory_change: false,
                controller_capability_change: false,
                submission: nvme::log::command_effects::Submission::Concurrent,
                uuid_selection: false,
            };
        const EFFECTS_LBA_CHANGE_TRUE: nvme::log::command_effects::Effects =
            nvme::log::command_effects::Effects {
                logical_block_change: true,
                namespace_capability_change: false,
                namespace_inventory_change: false,
                controller_capability_change: false,
                submission: nvme::log::command_effects::Submission::Concurrent,
                uuid_selection: false,
            };
        const ADMIN_VUCS: &[nvme::log::command_effects::AdminCommand] = &[
            nvme::log::command_effects::AdminCommand {
                opcode: nvme::command::AdminOpcode::VendorSpecific(0xD0),
                effects: EFFECTS_LBA_CHANGE_FALSE,
            },
            nvme::log::command_effects::AdminCommand {
                opcode: nvme::command::AdminOpcode::VendorSpecific(0xD1),
                effects: EFFECTS_LBA_CHANGE_TRUE,
            },
            nvme::log::command_effects::AdminCommand {
                opcode: nvme::command::AdminOpcode::VendorSpecific(0xD2),
                effects: EFFECTS_LBA_CHANGE_FALSE,
            },
            nvme::log::command_effects::AdminCommand {
                opcode: nvme::command::AdminOpcode::VendorSpecific(0xE0),
                effects: EFFECTS_LBA_CHANGE_FALSE,
            },
            nvme::log::command_effects::AdminCommand {
                opcode: nvme::command::AdminOpcode::VendorSpecific(0xE1),
                effects: EFFECTS_LBA_CHANGE_TRUE,
            },
            nvme::log::command_effects::AdminCommand {
                opcode: nvme::command::AdminOpcode::VendorSpecific(0xE2),
                effects: EFFECTS_LBA_CHANGE_FALSE,
            },
            nvme::log::command_effects::AdminCommand {
                opcode: nvme::command::AdminOpcode::VendorSpecific(0xF4),
                effects: EFFECTS_LBA_CHANGE_FALSE,
            },
        ];

        let valid = cmd_effects
            .admin
            .iter()
            .filter(|x| matches!(x.opcode, nvme::command::AdminOpcode::VendorSpecific(_)))
            .eq(ADMIN_VUCS);

        if let Some(drive) = drive {
            debug!("[{drive}] Validate command effects log: {valid}");
        }

        valid
    }

    /// Open drive.
    fn open(nvme: &'a nvme::Drive) -> Result<Option<Self>, Error> {
        const STATUS_AP_KEY_ERROR: nvme::status::StatusCode =
            nvme::status::StatusCode::VendorSpecific(0xEE);

        let mut drive = Self { nvme, hosin: false };

        let identify = drive.nvme.identify_controller()?;
        if !Self::validate_identify(&identify, Some(drive)) {
            return Ok(None);
        }

        let cmd_effects = nvme.command_effects()?;
        if !Self::validate_cmd_effects(&cmd_effects, Some(drive)) {
            return Ok(None);
        }

        // Check VUC system info
        let mut system_info_result = drive.vuc_system_info();

        // If AP key error, retry with Hosin magic
        if matches!(
            system_info_result,
            Err(Error::Nvme(nvme::Error::Status(x))) if x.status_code == STATUS_AP_KEY_ERROR
        ) {
            drive.hosin = true;
            system_info_result = drive.vuc_system_info();
        }

        debug!("[{drive}] Validate VUC system info: {system_info_result:?}");

        match system_info_result {
            Ok(_) => {},
            Err(Error::Nvme(x)) if x.is_command_error() => return Ok(None),
            Err(Error::InvalidSystemInfo) => return Ok(None),
            Err(x) => return Err(x),
        }

        debug!("[{drive}] Validated");
        Ok(Some(drive))
    }

    /// Execute VUC.
    fn vuc(
        self,
        transfer: Transfer,
        operation: VucOperation,
        parameters: [u32; 4],
    ) -> Result<(), nvme::Error> {
        self.nvme
            .admin_command(super::vuc_ap_key(self.hosin), Transfer::None, None)?;

        self.nvme
            .admin_command(super::vuc(&transfer, operation, parameters), transfer, None)?;

        Ok(())
    }

    /// VUC system info.
    fn vuc_system_info(self) -> Result<system_info::SystemInfo, Error> {
        let mut data = [0u8; system_info::SystemInfo::SIZE];

        self.vuc(Transfer::Read(&mut data), VucOperation::SystemInfo, [0; 4])?;

        let system_info = (&data).try_into()?;
        debug!("[{self}] System info: {system_info:?}");

        Ok(system_info)
    }

    /// VUC read info block.
    fn vuc_read_info_block(self) -> Result<info_block::InfoBlock, Error> {
        let mut data = [0u8; info_block::InfoBlock::SIZE];

        self.vuc(
            Transfer::Read(&mut data),
            VucOperation::ReadInfoBlock,
            [0; 4],
        )?;

        let info_block = (&data).try_into()?;
        debug!("[{self}] Info block: {info_block:?}");

        Ok(info_block)
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        self.vuc(
            Transfer::Read(data),
            VucOperation::ReadSram,
            [0, 0, address, 0],
        )?;
        Ok(())
    }

    /// Read ARM exception vector table from controller memory.
    fn read_exception_vector_table(self) -> Result<[u8; VECTOR_TABLE_SIZE], Error> {
        let mut data = [0; _];
        self.read_memory(EXCEPTION_VECTOR_TABLE_ADDRESS, &mut data)?;

        if !arm32::is_exception_vector_table(&data) {
            return Err(Error::InvalidExceptionVectorTable);
        }

        Ok(data)
    }
}

impl std::fmt::Display for Drive<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{DISPLAY_NAME} {}", self.nvme.path().display())
    }
}

/// E13 vendor type.
pub struct VendorType;

impl VendorType {
    /// Check read info block.
    fn check_read_info_block(drive: Drive) -> drive::CheckResult {
        let read_info_block_result = drive.vuc_read_info_block();
        debug!("[{drive}] Read info block: {read_info_block_result:?}");

        let result = match read_info_block_result {
            Ok(x) => Ok(format!("(serial: {}, model: {})", x.serial, x.model)),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Read info block".into(), result)
    }

    /// Check read controller memory.
    fn check_read_memory(drive: Drive) -> drive::CheckResult {
        let result = match drive.read_exception_vector_table() {
            Ok(_) => Ok(format!(
                "ARM exception vector table at {EXCEPTION_VECTOR_TABLE_ADDRESS:#010x}"
            )),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Read memory".into(), result)
    }
}

impl drive::VendorType for VendorType {
    fn name(&self) -> &str {
        DISPLAY_NAME
    }

    /// Run checks on drive.
    fn check(
        &self,
        drive: &drive::Drive,
    ) -> Result<Option<Box<[drive::CheckResult]>>, drive::Error> {
        // Drive must be NVMe
        let drive::Drive::Nvme(nvme_drive) = drive else {
            return Ok(None);
        };

        // Attempt to open drive
        let Some(drive) = Drive::open(nvme_drive)? else {
            return Ok(None);
        };

        let mut results = Vec::new();

        let system_info = drive.vuc_system_info()?;

        results.push(drive::CheckResult::new(
            "System info".into(),
            Ok(format!(
                "(firmware: {}-{} ({}), VUC lock: {}, CPU: {} MHz, flash: {} {}, CEs: {}, \
                 blocks per CE: {}, pages per block: {}, page size: {} KB)",
                system_info.firmware_version,
                system_info.firmware_subversion,
                system_info.firmware_date,
                system_info
                    .vuc_lock_state
                    .as_ref()
                    .map_or("N/A".into(), ToString::to_string),
                system_info.cpu_clock,
                system_info.flash_manufacturer,
                match system_info.flash_bits_per_cell {
                    1 => "SLC".to_string(),
                    2 => "MLC".to_string(),
                    3 => "TLC".to_string(),
                    4 => "QLC".to_string(),
                    x => format!("{x}-bit"),
                },
                system_info.ce_count,
                system_info.blocks_per_ce,
                system_info.pages_per_block,
                system_info.page_size,
            )),
        ));

        if matches!(
            system_info.vuc_lock_state,
            None | Some(VucLockState::Unlocked | VucLockState::NoLock)
        ) {
            // Only run other checks if VUCs unlocked
            results.push(Self::check_read_info_block(drive));
            results.push(Self::check_read_memory(drive));
        }

        Ok(Some(results.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn validate_identify() {
        const DATA_VALID: &[&[u8; identify::controller::Identify::SIZE]] =
            &[test_data::patriot_p300::IDENTIFY_CONTROLLER];

        for &data in DATA_VALID {
            let identify = identify::controller::Identify::try_from(data).unwrap();
            assert!(Drive::validate_identify(&identify, None));
        }
    }

    #[test]
    fn validate_cmd_effects() {
        const DATA_VALID: &[&[u8; nvme::log::command_effects::CommandEffects::SIZE]] =
            &[test_data::patriot_p300::LOG_5H];

        for &data in DATA_VALID {
            let cmd_effects = nvme::log::command_effects::CommandEffects::try_from(data).unwrap();
            assert!(Drive::validate_cmd_effects(&cmd_effects, None));
        }
    }
}
