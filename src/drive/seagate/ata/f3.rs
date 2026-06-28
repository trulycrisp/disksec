//! F3 architecture.

use log::{debug, info};

use super::super::sdbp;
use crate::{
    drive,
    protocol::{
        Transfer,
        ata::{
            self, SECTOR_SIZE,
            command::{Command, CommandRegisters},
            identify,
        },
        scsi::sense,
    },
};

/// Memory address of ARM exception vector.
const EXCEPTION_VECTOR_ADDRESS: u32 = 0;
/// ARM 32-bit instruction size.
const INSTRUCTION_SIZE: usize = 4;

/// F3 error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// Invalid firmware information (from identify device VUC).
    InvalidFirmwareInfo,
    /// SDBP error.
    Sdbp(sdbp::Error),
    /// SDBP packet has unexpected port.
    UnexpectedSdbpPort(sdbp::Port),
    /// Diagnostic Internal Test Service error.
    Dits(sense::SenseKey, sense::asc::AdditionalSenseCode),
    /// Data returned from VUC is too short.
    VucDataTruncated,
    /// Invalid ARM exception vector in memory.
    InvalidExceptionVector,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ata(x) => Some(x),
            Self::Sdbp(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::InvalidFirmwareInfo => write!(f, "invalid firmware information"),
            Self::Sdbp(_) => write!(f, "SDBP error"),
            Self::UnexpectedSdbpPort(x) => write!(f, "unexpected SDBP port {x}"),
            Self::Dits(key, asc) => write!(f, "DITS {key} {asc}"),
            Self::VucDataTruncated => write!(f, "VUC data truncated"),
            Self::InvalidExceptionVector => write!(f, "invalid ARM exception vector"),
        }
    }
}
impl drive::VendorError for Error {}

impl From<ata::Error> for Error {
    fn from(value: ata::Error) -> Self {
        Self::Ata(value)
    }
}

impl From<sdbp::Error> for Error {
    fn from(value: sdbp::Error) -> Self {
        Self::Sdbp(value)
    }
}

impl From<Error> for drive::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Ata(x) => Self::Ata(x),
            x => Self::Vendor(Box::new(x)),
        }
    }
}

/// Firmware information, from identify device VUC variant.
#[derive(Clone, Debug, PartialEq, Eq)]
struct FirmwareInfo {
    /// Package version.
    version: String,
    /// Package build timestamp.
    build_time: String,
}

impl FirmwareInfo {
    /// Size in bytes.
    const SIZE: usize = SECTOR_SIZE;

    /// Parse string.
    fn parse_str(mut data: &[u8]) -> Result<&str, Error> {
        // Trim trailing null bytes
        data = data
            .iter()
            .rposition(|&x| x != 0)
            .map(|x| &data[..=x])
            .ok_or(Error::InvalidFirmwareInfo)?;

        data.iter()
            .all(|x| x.is_ascii_graphic() || x.is_ascii_whitespace())
            .then(|| std::str::from_utf8(data).unwrap())
            .ok_or(Error::InvalidFirmwareInfo)
    }

    /// Parse timestamp.
    fn parse_time(data: &[u8]) -> Result<String, Error> {
        const SIZE: usize = 14;

        let string = Self::parse_str(data)?;

        if string.len() != SIZE {
            return Err(Error::InvalidFirmwareInfo);
        }

        Ok(format!(
            "{}-{}-{} {}:{}:{}",
            &string[..4],
            &string[4..6],
            &string[6..8],
            &string[8..10],
            &string[10..12],
            &string[12..]
        ))
    }
}

impl TryFrom<&[u8; Self::SIZE]> for FirmwareInfo {
    type Error = Error;

    fn try_from(value: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        let version = Self::parse_str(&value[..32])?.trim_end().into();
        let build_time = Self::parse_time(&value[112..126])?;

        Ok(Self {
            version,
            build_time,
        })
    }
}

/// Drive interface.
#[derive(Clone, Copy, Debug)]
struct Drive<'a> {
    /// ATA drive.
    ata: &'a ata::Drive,
}

impl Drive<'_> {
    /// GPL log for VUC read operations.
    const VUC_LOG_READ: ata::log::Log = ata::log::Log::VendorSpecific(0xBF);
    /// GPL log for VUC write operations.
    const VUC_LOG_WRITE: ata::log::Log = ata::log::Log::VendorSpecific(0xBE);

    /// Validate identify device result matches supported drive.
    fn validate_identify(identify: &identify::IdentifyDevice, drive: Option<Self>) -> bool {
        const MODEL_PREFIX: &str = "ST";
        const WWN_OUI: u32 = 0xC50;

        let is_hdd = matches!(identify.rotation_rate, Some(identify::RotationRate::Rpm(_)));
        let model_match = identify.model.starts_with(MODEL_PREFIX);
        let wwn_match = identify.wwn.is_some_and(|x| x.oui == WWN_OUI);
        let has_gpl = identify.features_enabled.gpl == Some(true);
        let valid = is_hdd && model_match && wwn_match && has_gpl;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (HDD: {is_hdd}, model: {model_match}, WWN: \
                 {wwn_match}, GPL: {has_gpl})"
            );
        }

        valid
    }

    /// Validate supported drive.
    fn validate(self) -> Result<bool, Error> {
        const VUC_LOG_PAGE_COUNT: u16 = u16::MAX;

        // Check identify device fields
        let identify = self.ata.identify_device()?;
        if !Self::validate_identify(&identify, Some(self)) {
            return Ok(false);
        }

        let gpl_directory = self.ata.gpl_directory()?;
        let has_vuc_log_write = gpl_directory[Self::VUC_LOG_WRITE] == VUC_LOG_PAGE_COUNT;
        let has_vuc_log_read = gpl_directory[Self::VUC_LOG_READ] == VUC_LOG_PAGE_COUNT;
        let has_vuc_logs = has_vuc_log_write && has_vuc_log_read;
        debug!(
            "[{self}] Validate VUC logs: {has_vuc_logs} (write: {has_vuc_log_write}, read: \
             {has_vuc_log_read})"
        );
        if !has_vuc_logs {
            return Ok(false);
        }

        let firmware_info_result = self.firmware_info();
        debug!("[{self}] Validate firmware information: {firmware_info_result:?}");
        let firmware_info = match firmware_info_result {
            Ok(_) => true,
            Err(Error::Ata(x)) if x.is_command_error() => false,
            Err(Error::InvalidFirmwareInfo) => false,
            Err(x) => return Err(x),
        };
        if !firmware_info {
            return Ok(false);
        }

        debug!("[{self}] Validated");

        Ok(true)
    }

    /// Get firmware information, with identify device VUC variant.
    fn firmware_info(self) -> Result<FirmwareInfo, Error> {
        const REGISTERS: CommandRegisters = CommandRegisters {
            feature: 0xB,
            count: 1,
            lba: 0x0020_97DB,
            device: 0,
            command: Command::IdentifyDevice,
        };

        let mut data = [0u8; SECTOR_SIZE];

        self.ata
            .command(REGISTERS, Transfer::Read(&mut data), false, false, None)?;

        let firmware_info = (&data).try_into()?;
        debug!("[{self}] Firmware information: {firmware_info:?}");

        Ok(firmware_info)
    }

    /// Execute VUC.
    fn vuc(
        self,
        port: sdbp::Port,
        dfb: &sdbp::Dfb,
        max_result_size: usize,
    ) -> Result<Box<[u8]>, Error> {
        const LOG_PAGE: u16 = 0x2459;

        let log_info = format!(
            "function: {:#x}, revision: {:#x}, data size: {}",
            dfb.function,
            dfb.revision,
            dfb.data.len()
        );
        debug!("[{self}] Executing VUC: {port} ({log_info}, max result size: {max_result_size})");

        let dfb = Box::<[_]>::from(dfb);
        let write_packet = sdbp::Packet {
            from: port,
            to: port,
            data: &dfb,
        };
        let write_packet = Box::<[_]>::from(&write_packet);
        self.ata
            .gpl_write(&write_packet, Self::VUC_LOG_WRITE, LOG_PAGE, 0)?;

        let read_size = sdbp::Packet::HEADER_SIZE + max_result_size;
        let mut read_packet = vec![0u8; read_size].into_boxed_slice();
        self.ata
            .gpl_read(&mut read_packet, Self::VUC_LOG_READ, LOG_PAGE, 0)?;
        let read_packet = sdbp::Packet::try_from(read_packet.as_ref())?;

        if read_packet.from != port {
            return Err(Error::UnexpectedSdbpPort(read_packet.from));
        }

        debug!(
            "[{self}] Executed VUC: {port} ({log_info}, result size: {})",
            read_packet.data.len()
        );

        Ok(read_packet.data.into())
    }

    /// Execute Diagnostic Internal Test Service VUC.
    fn vuc_dits(self, dfb: &sdbp::Dfb, result: &mut [u8]) -> Result<(), Error> {
        let max_dsb_size = sdbp::DitsDsb::HEADER_SIZE + result.len();
        let dsb = self.vuc(sdbp::Port::Dits, dfb, max_dsb_size)?;
        let dsb = sdbp::DitsDsb::try_from(dsb.as_ref())?;

        info!(
            "[{self}] Executed DITS VUC: {:#x} {:#x} (sense key: {}, ASC/ASCQ: {}, result size: \
             {})",
            dfb.function,
            dfb.revision,
            dsb.sense_key,
            dsb.asc,
            dsb.data.len()
        );

        // Check error status before data, error only returns DSB header
        if dsb.sense_key.is_error() {
            return Err(Error::Dits(dsb.sense_key, dsb.asc));
        }

        if dsb.data.len() != result.len() {
            return Err(Error::VucDataTruncated);
        }

        result.copy_from_slice(dsb.data);

        Ok(())
    }

    /// VUC unlock Diagnostic Internal Test Service
    fn vuc_dits_unlock(self) -> Result<(), Error> {
        const FUNCTION: u16 = 0xFFFF;
        const REVISION: u16 = 1;
        const KEY: &[u8] = &[0x9A, 0x32, 0x4F, 0x03];

        let function = sdbp::Dfb {
            function: FUNCTION,
            revision: REVISION,
            data: KEY,
        };

        self.vuc_dits(&function, &mut [])?;

        Ok(())
    }

    /// VUC DITS read system file (system area module).
    fn vuc_dits_read_system_file(
        self,
        id: u8,
        sector_count: u16,
        sector_offset: u16,
        get_sector_count: bool,
    ) -> Result<Box<[u8]>, Error> {
        const FUNCTION: u16 = 0x144;
        const REVISION: u16 = 1;

        let dfb_sector_count = sector_count.to_le_bytes();
        let dfb_sector_offset = sector_offset.to_le_bytes();
        let dfb_data = [
            id,
            0,
            dfb_sector_count[0],
            dfb_sector_count[1],
            dfb_sector_offset[0],
            dfb_sector_offset[1],
            0,
            0,
            get_sector_count.into(),
            0,
            0,
            0,
        ];
        let dfb = sdbp::Dfb {
            function: FUNCTION,
            revision: REVISION,
            data: &dfb_data,
        };

        let result_size = if get_sector_count {
            size_of::<u32>()
        } else {
            sector_count as usize * SECTOR_SIZE
        };

        let mut result = vec![0u8; result_size].into_boxed_slice();

        self.vuc_dits(&dfb, &mut result)?;

        Ok(result)
    }

    /// Get size of a system file in sectors.
    fn system_file_sector_count(self, id: u8) -> Result<u16, Error> {
        let data = self.vuc_dits_read_system_file(id, 0, 0, true)?;
        let sector_count = u16::from_le_bytes([data[0], data[1]]);

        debug!("[{self}] System file {id:#x} sector count: {sector_count}");

        Ok(sector_count)
    }

    /// Read a system file.
    fn system_file_read(self, id: u8) -> Result<Box<[u8]>, Error> {
        let sector_count = self.system_file_sector_count(id)?;
        self.vuc_dits_read_system_file(id, sector_count, 0, false)
    }

    /// Enumerate present system files.
    fn system_file_list(self) -> Result<Box<[u8]>, Error> {
        const MAX_ID: u8 = u8::MAX;

        let mut list = Vec::with_capacity(MAX_ID as usize + 1);

        for id in 0..=MAX_ID {
            let sector_count = match self.system_file_sector_count(id) {
                Ok(x) => x,
                Err(Error::Dits(
                    sense::SenseKey::IllegalRequest,
                    sense::asc::AdditionalSenseCode::ParameterValueInvalid,
                )) => 0,
                Err(x) => return Err(x),
            };

            if sector_count > 0 {
                list.push(id);
            }
        }

        Ok(list.into())
    }

    /// VUC DITS read controller memory.
    fn vuc_dits_read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        const FUNCTION: u16 = 0x149;
        const REVISION: u16 = 1;

        let dfb_address = address.to_le_bytes();
        let dfb_size = u16::try_from(data.len()).unwrap().to_le_bytes();
        let dfb_data = [
            dfb_address[0],
            dfb_address[1],
            dfb_address[2],
            dfb_address[3],
            0,
            0,
            dfb_size[0],
            dfb_size[1],
            0,
            0,
            0,
            0,
        ];
        let dfb = sdbp::Dfb {
            function: FUNCTION,
            revision: REVISION,
            data: &dfb_data,
        };

        self.vuc_dits(&dfb, data)
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        const MAX_READ_SIZE: usize = u16::MAX as _;

        for (chunk_index, chunk_data) in data.chunks_mut(MAX_READ_SIZE).enumerate() {
            let read_address = address + u32::try_from(chunk_index * MAX_READ_SIZE).unwrap();
            self.vuc_dits_read_memory(read_address, chunk_data)?;
        }

        Ok(())
    }

    /// Check data is an ARM exception vector
    fn is_exception_vector(data: &[u8; SECTOR_SIZE]) -> bool {
        const LDR_PC: &[u8; INSTRUCTION_SIZE] = &[0x0, 0xF0, 0x9F, 0xE5]; // ldr pc, [pc, #<x>]
        const LDR_PC_MASK: &[u8; INSTRUCTION_SIZE] = &[0x0, 0xFF, 0xFF, 0xFF];
        const INSTRUCTION_COUNT: usize = 3;

        for instruction_index in 0..INSTRUCTION_COUNT {
            let instruction_offset = instruction_index * INSTRUCTION_SIZE;
            let instruction = &data[instruction_offset..instruction_offset + INSTRUCTION_SIZE];

            if instruction
                .iter()
                .zip(LDR_PC)
                .zip(LDR_PC_MASK)
                .any(|((x, y), m)| x & m != y & m)
            {
                return false;
            }
        }

        true
    }

    /// Read ARM exception vector from controller memory.
    fn read_exception_vector(self) -> Result<[u8; SECTOR_SIZE], Error> {
        let mut data = [0; _];
        self.read_memory(EXCEPTION_VECTOR_ADDRESS, &mut data)?;

        if !Self::is_exception_vector(&data) {
            return Err(Error::InvalidExceptionVector);
        }

        Ok(data)
    }
}

impl std::fmt::Display for Drive<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Seagate F3 {}", self.ata.path().display())
    }
}

/// F3 vendor type.
pub struct VendorType;

impl VendorType {
    /// Check firmware information, from identify device VUC variant.
    fn check_firmware_info(drive: Drive) -> Result<drive::CheckResult, Error> {
        let firmware_info = drive.firmware_info()?;

        Ok(drive::CheckResult {
            name: "Firmware information".into(),
            result: format!("{} ({})", firmware_info.version, firmware_info.build_time),
        })
    }

    /// Check reading system area.
    fn check_read_sa(drive: Drive) -> drive::CheckResult {
        let system_file_list_result = drive.system_file_list();
        debug!("[{drive}] System file list: {system_file_list_result:?}");

        let result = match system_file_list_result {
            Ok(x) => format!("Success ({} modules)", x.len()),
            Err(x) => format!("Fail ({x})"),
        };

        drive::CheckResult {
            name: "Read system area".into(),
            result,
        }
    }

    /// Check reading controller memory.
    fn check_read_memory(drive: Drive) -> drive::CheckResult {
        let read_exception_vector_result = drive.read_exception_vector();
        debug!(
            "[{drive}] Read exception vector (check read memory): {read_exception_vector_result:?}"
        );

        let result = match read_exception_vector_result {
            Ok(_) => format!("Success (ARM exception vector at {EXCEPTION_VECTOR_ADDRESS:#010x})"),
            Err(x) => format!("Fail ({x})"),
        };

        drive::CheckResult {
            name: "Read memory".into(),
            result,
        }
    }

    /// Check for IRATEMONK infection, implements the same logic `nls_933w.dll`
    /// uses to locate implant data storage.
    fn check_iratemonk(drive: Drive) -> drive::CheckResult {
        const RESERVED_VECTOR_OFFSET: usize = 5 * INSTRUCTION_SIZE;
        const MOV_R0_R0: &[u8; INSTRUCTION_SIZE] = &[0x0, 0x0, 0xA0, 0xE1];
        const NOP: &[u8; INSTRUCTION_SIZE] = &[0x0, 0xF0, 0x20, 0xE3];

        let result = |x| drive::CheckResult {
            name: "IRATEMONK".into(),
            result: x,
        };

        let read_exception_vector_result = drive.read_exception_vector();
        debug!(
            "[{drive}] Read exception vector (check IRATEMONK): {read_exception_vector_result:?}"
        );

        let data = match read_exception_vector_result {
            Ok(x) => x,
            Err(x) => return result(format!("Fail ({x})")),
        };

        let reserved_vector: [_; INSTRUCTION_SIZE] =
            std::array::from_fn(|x| data[RESERVED_VECTOR_OFFSET + x]);

        if &reserved_vector == MOV_R0_R0 || &reserved_vector == NOP {
            return result("Infection not found".into());
        }

        let data_address = u32::from_le_bytes(reserved_vector);

        result(format!("INFECTED (data address: {data_address:#x})"))
    }
}

impl drive::VendorType for VendorType {
    fn name(&self) -> &str {
        const NAME: &str = "Seagate F3";

        NAME
    }

    fn check(
        &self,
        drive: &drive::Drive,
    ) -> Result<Option<Box<[drive::CheckResult]>>, drive::Error> {
        let drive = match drive {
            drive::Drive::Ata(ata) => Drive { ata },
            drive::Drive::Scsi(_) => return Ok(None),
        };

        if !drive.validate()? {
            return Ok(None);
        }

        drive.vuc_dits_unlock()?;

        let results = vec![
            Self::check_firmware_info(drive)?,
            Self::check_read_sa(drive),
            Self::check_read_memory(drive),
            Self::check_iratemonk(drive),
        ]
        .into();

        Ok(Some(results))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn validate_identify() {
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::seagate_momentus5::IDENTIFY,
            test_data::seagate_barracudapro::IDENTIFY,
        ];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::westerndigital_scorpioblack::IDENTIFY,
            test_data::seagate_momentus3::IDENTIFY, // ST-10
        ];

        for &data in DATA_VALID {
            let identify = identify::IdentifyDevice::try_from(data).unwrap();
            assert!(Drive::validate_identify(&identify, None));
        }

        for &data in DATA_INVALID {
            let identify = identify::IdentifyDevice::try_from(data).unwrap();
            assert!(!Drive::validate_identify(&identify, None));
        }
    }

    #[test]
    fn parse_firmware_info() {
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::seagate_momentus5::FIRMWARE_INFO,
            test_data::seagate_barracudapro::FIRMWARE_INFO,
        ];
        const DATA_INVALID: &[&[u8; FirmwareInfo::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(FirmwareInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(FirmwareInfo::try_from(data).is_err());
        }
    }

    #[test]
    fn is_exception_vector() {
        const DATA_VALID: &[u8; SECTOR_SIZE] = test_data::seagate_momentus5::EXCEPTION_VECTOR;
        const DATA_INVALID: &[&[u8; SECTOR_SIZE]] = &[&[0; _], &[0xFF; _]];

        assert!(Drive::is_exception_vector(DATA_VALID));

        for &data in DATA_INVALID {
            assert!(!Drive::is_exception_vector(data));
        }
    }
}
