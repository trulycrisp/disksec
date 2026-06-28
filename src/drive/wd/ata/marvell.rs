//! Marvell controller architecture.

use log::{debug, info};

use crate::{
    drive,
    protocol::{
        Transfer,
        ata::{
            self, SECTOR_SIZE,
            command::{Command, CommandRegisters, ResultRegisters},
            identify,
        },
    },
};

/// Memory address of ARM exception vector.
const EXCEPTION_VECTOR_ADDRESS: u32 = 0;
/// ARM 32-bit instruction size.
const INSTRUCTION_SIZE: usize = 4;

/// Marvell error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// Invalid header of file (system area module).
    InvalidFileHeader,
    /// Invalid data of file (system area module) directory.
    InvalidFileDirectory,
    /// Invalid VUC native information data.
    InvalidNativeInfo,
    /// Invalid exception vector data in memory.
    InvalidExceptionVector,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ata(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::InvalidFileHeader => write!(f, "invalid file header"),
            Self::InvalidFileDirectory => write!(f, "invalid file directory"),
            Self::InvalidNativeInfo => write!(f, "invalid native info"),
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

impl From<Error> for drive::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Ata(x) => Self::Ata(x),
            x => Self::Vendor(Box::new(x)),
        }
    }
}

/// VUC action code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VucAction {
    /// File (system area module).
    File = 8,
    /// Information table.
    Table = 13,
    /// Controller memory.
    Memory = 19,
}

impl std::fmt::Display for VucAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::Table => write!(f, "table"),
            Self::Memory => write!(f, "memory"),
        }
    }
}

/// VUC function code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VucFunction {
    /// Read data.
    Read = 1,
    /// Write data.
    Write = 2,
}

/// Header of file (system area module).
#[derive(Clone, Debug, PartialEq, Eq)]
struct FileHeader {
    /// Header size in bytes.
    header_size: u16,
    /// File ID.
    id: u16,
    /// Total file size in sectors, including header.
    sector_count: u16,
    /// Checksum value.
    checksum: u32,
    /// File data format version/revision.
    version: String,
}

impl FileHeader {
    /// Maximum size in bytes.
    const MAX_SIZE: usize = SECTOR_SIZE;
    /// Minimum size in bytes.
    const MIN_SIZE: usize = 24;
}

impl TryFrom<&[u8]> for FileHeader {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        const MAGIC: &[u8] = b"ROYL";

        if value.len() < Self::MIN_SIZE || !value.starts_with(MAGIC) {
            return Err(Error::InvalidFileHeader);
        }

        let header_size = u16::from_le_bytes([value[6], value[7]]);
        if !(Self::MIN_SIZE..=Self::MAX_SIZE.min(value.len())).contains(&(header_size as usize)) {
            return Err(Error::InvalidFileHeader);
        }

        let id = u16::from_le_bytes([value[8], value[9]]);

        let sector_count = u16::from_le_bytes([value[10], value[11]]);
        if sector_count == 0 {
            return Err(Error::InvalidFileHeader);
        }

        let checksum = u32::from_le_bytes([value[12], value[13], value[14], value[15]]);
        let version = std::str::from_utf8(&value[16..24])
            .or(Err(Error::InvalidFileHeader))?
            .into();

        Ok(Self {
            header_size,
            id,
            sector_count,
            checksum,
            version,
        })
    }
}

/// File (system area module).
#[derive(Clone, Debug, PartialEq, Eq)]
struct File {
    /// File header.
    header: FileHeader,
    /// File data.
    data: Box<[u8]>,
}

impl TryFrom<&[u8]> for File {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let header = FileHeader::try_from(value)?;
        let data = (value[header.header_size as _..]).into();

        Ok(Self { header, data })
    }
}

/// Directory of files (system area modules).
#[derive(Clone, Debug, PartialEq, Eq)]
struct FileDirectory {
    /// File IDs in directory.
    entries: Box<[u16]>,
}

impl TryFrom<&File> for FileDirectory {
    type Error = Error;

    fn try_from(file: &File) -> Result<Self, Self::Error> {
        const VERSION: &str = "00020000";
        const MIN_ENTRY_SIZE: usize = 18;

        // Validate directory version/format
        if file.header.version != VERSION {
            return Err(Error::InvalidFileDirectory);
        }

        let (entry_count, mut data) = file
            .data
            .split_first_chunk::<{ size_of::<u16>() }>()
            .ok_or(Error::InvalidFileDirectory)?;
        let entry_count = u16::from_le_bytes(*entry_count) as usize;

        // Get entry size from first entry
        let entry_size = *data.first().ok_or(Error::InvalidFileDirectory)? as _;
        if entry_size < MIN_ENTRY_SIZE {
            return Err(Error::InvalidFileDirectory);
        }

        // Truncate data to entries
        data = data
            .get(..entry_count * entry_size)
            .ok_or(Error::InvalidFileDirectory)?;

        // Parse entries
        let mut entries = Vec::<u16>::with_capacity(entry_count as _);
        for entry_data in data.chunks_exact(entry_size) {
            let file_id = u16::from_le_bytes([entry_data[2], entry_data[3]]);

            // Skip empty entries
            if file_id == 0 {
                continue;
            }

            // Check for duplicate entry
            if entries.contains(&file_id) {
                return Err(Error::InvalidFileDirectory);
            }

            entries.push(file_id);
        }

        Ok(Self {
            entries: entries.into(),
        })
    }
}

impl TryFrom<&[u8]> for FileDirectory {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        (&File::try_from(value)?).try_into()
    }
}

/// VUC native information table.
#[derive(Clone, Debug, PartialEq, Eq)]
struct NativeInfo {
    /// Controller firmware version.
    controller_firmware: String,
    /// Servo firmware version.
    servo_firmware: String,
    /// Overlay firmware version.
    overlay_firmware: String,
    /// Read-channel firmware version.
    read_channel_firmware: String,
    /// Drive model.
    model: String,
    /// Drive Configuration Matrix.
    drive_config_matrix: String,
    /// Manufacture date.
    manufacture_date: String,
    /// Drive serial number.
    serial: String,
}

impl NativeInfo {
    /// Size in bytes.
    const SIZE: usize = SECTOR_SIZE;

    /// Parse string.
    fn parse_str(data: &[u8]) -> Result<&str, Error> {
        data.iter()
            .all(|x| x.is_ascii_graphic() || x.is_ascii_whitespace())
            .then(|| std::str::from_utf8(data).unwrap())
            .ok_or(Error::InvalidNativeInfo)
    }
}

impl TryFrom<&[u8; Self::SIZE]> for NativeInfo {
    type Error = Error;

    fn try_from(value: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        const FORMAT_VERSION: u8 = 2;

        let format_version = value[0];
        if format_version != FORMAT_VERSION {
            return Err(Error::InvalidNativeInfo);
        }

        let controller_firmware = Self::parse_str(&value[2..8])?.into();
        let servo_firmware = Self::parse_str(&value[10..15])?.into();
        let overlay_firmware = Self::parse_str(&value[18..24])?.into();
        let read_channel_firmware = Self::parse_str(&value[86..94])?.into();
        let model = Self::parse_str(&value[210..250])?.trim_end().into();
        let drive_config_matrix = Self::parse_str(&value[256..278])?.into();
        let manufacture_date = Self::parse_str(&value[292..302])?.into();
        let serial = Self::parse_str(&value[318..333])?.into();

        Ok(Self {
            controller_firmware,
            servo_firmware,
            overlay_firmware,
            read_channel_firmware,
            model,
            drive_config_matrix,
            manufacture_date,
            serial,
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
    /// Validate identify device result matches supported drive.
    fn validate_identify(identify: &identify::IdentifyDevice, drive: Option<Self>) -> bool {
        const MODEL_PREFIX: &str = "WDC WD";
        const WWN_OUI: u32 = 0x14EE;

        let is_hdd = matches!(identify.rotation_rate, Some(identify::RotationRate::Rpm(_)));
        let model_match = identify.model.starts_with(MODEL_PREFIX);
        let wwn_match = identify.wwn.is_some_and(|x| x.oui == WWN_OUI);
        let smart_log_support = identify.features_supported.smart == Some(true)
            && identify.features_supported.smart_error_log == Some(true);
        let valid = is_hdd && model_match && wwn_match && smart_log_support;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (HDD: {is_hdd}, model: {model_match}, WWN: \
                 {wwn_match}, SMART log: {smart_log_support})"
            );
        }

        valid
    }

    /// Validate supported drive.
    fn validate(self) -> Result<bool, Error> {
        // Check identify device fields
        let identify = self.ata.identify_device()?;
        if !Self::validate_identify(&identify, Some(self)) {
            return Ok(false);
        }

        // Check VUC enable
        let vuc_enable = match self.vuc_set_enabled(true) {
            Ok(()) => true,
            Err(Error::Ata(x)) if x.is_command_error() => false,
            Err(x) => return Err(x),
        };
        debug!("[{self}] Validate VUC enable: {vuc_enable}");
        if !vuc_enable {
            return Ok(false);
        }

        debug!("[{self}] Validated");

        // Undo the VUC enable
        self.vuc_set_enabled(false)?;

        Ok(true)
    }

    /// Enable or disable VUC access.
    fn vuc_set_enabled(self, enable: bool) -> Result<(), Error> {
        const FEATURE_ENABLE: u16 = 'E' as _;
        const FEATURE_DISABLE: u16 = 'D' as _;
        const LBA: u64 = (('W' as u64) << 16) | (('D' as u64) << 8);
        const COMMAND: Command = Command::VendorSpecific(0x80);

        let feature = if enable {
            FEATURE_ENABLE
        } else {
            FEATURE_DISABLE
        };

        let registers = CommandRegisters {
            feature,
            lba: LBA,
            command: COMMAND,
            ..Default::default()
        };

        self.ata
            .command(registers, Transfer::None, false, false, None)?;

        Ok(())
    }

    /// Execute VUC, uses variant of SMART command transport.
    fn vuc(
        self,
        transfer: Transfer,
        action: VucAction,
        function: u16,
        parameter: &[u8],
    ) -> Result<Option<ResultRegisters>, Error> {
        const SMART_LOG_KEY: ata::log::Log = ata::log::Log::VendorSpecific(0xBE);
        const SMART_LOG_DATA: ata::log::Log = ata::log::Log::VendorSpecific(0xBF);

        let key_action = action as u16;
        let key_data_size = size_of_val(&key_action) + size_of_val(&function) + parameter.len();
        let mut key_data = Vec::with_capacity(key_data_size);
        key_data.extend_from_slice(&key_action.to_le_bytes());
        key_data.extend_from_slice(&function.to_le_bytes());
        key_data.extend_from_slice(parameter);

        let log_info = format!("action: {action}, function: {function}, data: {transfer}");
        debug!("[{self}] Executing VUC: ({log_info})");
        self.ata.smart_log_write(&key_data, SMART_LOG_KEY)?;

        let result_registers = match transfer {
            Transfer::None => self.ata.smart_log_write(&[0u8], SMART_LOG_DATA),
            Transfer::Read(x) => self.ata.smart_log_read(x, SMART_LOG_DATA),
            Transfer::Write(x) => self.ata.smart_log_write(x, SMART_LOG_DATA),
        }?;

        info!("[{self}] Executed VUC: ({log_info})");
        Ok(result_registers)
    }

    /// Read file (system area module).
    fn read_file(self, id: u16) -> Result<File, Error> {
        let parameter = id.to_le_bytes();

        // Read header
        let mut header_data = [0u8; FileHeader::MAX_SIZE];
        self.vuc(
            Transfer::Read(&mut header_data),
            VucAction::File,
            VucFunction::Read as _,
            &parameter,
        )?;

        let header = FileHeader::try_from(header_data.as_slice())?;
        debug!("[{self}] Read file {id} header: {header:?}");

        // Read data
        let data_size = (header.sector_count as usize) * SECTOR_SIZE;
        let mut data = vec![0u8; data_size].into_boxed_slice();
        self.vuc(
            Transfer::Read(&mut data),
            VucAction::File,
            VucFunction::Read as _,
            &parameter,
        )?;

        data.as_ref().try_into()
    }

    /// Read directory of present files (system area modules).
    fn read_file_directory(self) -> Result<FileDirectory, Error> {
        /// Well-known file id of the directory file itself.
        const FILE_ID_DIRECTORY: u16 = 1;

        let file = self.read_file(FILE_ID_DIRECTORY)?;
        (&file).try_into()
    }

    /// VUC read table.
    fn get_table(self, data: &mut [u8], id: u16) -> Result<(), Error> {
        self.vuc(Transfer::Read(data), VucAction::Table, id, &[])?;

        Ok(())
    }

    /// VUC native information table.
    fn native_info(self) -> Result<NativeInfo, Error> {
        const TABLE_ID: u16 = 1;

        let mut data = [0u8; SECTOR_SIZE];
        self.get_table(&mut data, TABLE_ID)?;

        let native_info = (&data).try_into()?;
        debug!("[{self}] Native info: {native_info:?}");

        Ok(native_info)
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        let size = u32::try_from(data.len()).unwrap();
        let mut parameter = Vec::with_capacity(size_of_val(&address) + size_of_val(&size));
        parameter.extend_from_slice(&address.to_le_bytes());
        parameter.extend_from_slice(&size.to_le_bytes());

        self.vuc(
            Transfer::Read(data),
            VucAction::Memory,
            VucFunction::Read as _,
            &parameter,
        )?;

        Ok(())
    }

    /// Check data is an ARM exception vector.
    fn is_exception_vector(data: &[u8; SECTOR_SIZE]) -> bool {
        const LDR_PC: &[u8; INSTRUCTION_SIZE] = &[0x0, 0xF0, 0x9F, 0xE5];
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
        write!(f, "Western Digital Marvell {}", self.ata.path().display())
    }
}

/// Marvell vendor type.
pub struct VendorType;

impl VendorType {
    /// Check VUC native information table.
    fn check_native_info(drive: Drive) -> drive::CheckResult {
        let result = match drive.native_info() {
            Ok(x) => format!(
                "Success (firmware: (controller: {}, servo: {}, overlay: {}, read channel: {}), \
                 DCM: {}, manufacture date: {})",
                x.controller_firmware,
                x.servo_firmware,
                x.overlay_firmware,
                x.read_channel_firmware,
                x.drive_config_matrix,
                x.manufacture_date
            ),
            Err(x) => format!("Fail ({x})"),
        };

        drive::CheckResult {
            name: "Native information".into(),
            result,
        }
    }

    /// Check read system area.
    fn check_read_sa(drive: Drive) -> drive::CheckResult {
        let result = match drive.read_file_directory() {
            Ok(x) => format!("Success ({} modules)", x.entries.len()),
            Err(x) => format!("Fail ({x})"),
        };

        drive::CheckResult {
            name: "Read system area".into(),
            result,
        }
    }

    /// Check read controller memory.
    fn check_read_memory(drive: Drive) -> drive::CheckResult {
        let result = match drive.read_exception_vector() {
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
        const FIQ_VECTOR_OFFSET: usize = 28;
        const MOV_R0_R0: [u8; INSTRUCTION_SIZE] = [0x0, 0x0, 0xA0, 0xE1]; // mov r0, r0
        const MOV_R1_R1: [u8; INSTRUCTION_SIZE] = [0x1, 0x10, 0xA0, 0xE1]; // mov r1, r1
        const MOV_R2_R2: [u8; INSTRUCTION_SIZE] = [0x2, 0x20, 0xA0, 0xE1]; // mov r2, r2
        const MOV_R3_R3: [u8; INSTRUCTION_SIZE] = [0x3, 0x30, 0xA0, 0xE1]; // mov r3, r3
        const LDR_PC_18H: [u8; INSTRUCTION_SIZE] = [0x18, 0xF0, 0x9F, 0xE5]; // ldr pc, [pc, #0x18]

        let result = |x| drive::CheckResult {
            name: "IRATEMONK".into(),
            result: x,
        };
        let result_not_found = "Infection not found".to_string();

        let data = match drive.read_exception_vector() {
            Ok(x) => x,
            Err(x) => return result(format!("Fail ({x})")),
        };

        let reserved_vector = std::array::from_fn(|x| data[RESERVED_VECTOR_OFFSET + x]);
        let fiq_vector = std::array::from_fn(|x| data[FIQ_VECTOR_OFFSET + x]);

        let chs_packed_offset;
        if reserved_vector == MOV_R0_R0 && fiq_vector == LDR_PC_18H {
            chs_packed_offset = 52;
        } else if reserved_vector == MOV_R1_R1 {
            chs_packed_offset = 60;
        } else if reserved_vector == MOV_R2_R2 {
            chs_packed_offset = 180;
        } else if reserved_vector == MOV_R3_R3 {
            chs_packed_offset = 92;
        } else {
            return result(result_not_found);
        }

        let chs_packed = u32::from_le_bytes(std::array::from_fn(|x| data[chs_packed_offset + x]));
        debug!("[{drive}] IRATEMONK packed CHS {chs_packed:#x} (offset: {chs_packed_offset})");

        // Valid packed CHS must have high bit set
        if chs_packed & (1 << 31) == 0 {
            return result(result_not_found);
        }

        let cylinder = chs_packed.cast_signed() >> 15;
        let head = ((chs_packed >> 12) & 0b111) as u8;
        let sector = (chs_packed & 0xFFF) as u16;

        result(format!("INFECTED (data C/H/S: {cylinder}/{head}/{sector})"))
    }
}

impl drive::VendorType for VendorType {
    fn name(&self) -> &str {
        const NAME: &str = "Western Digital Marvell";

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

        // ensure SMART enabled (for VUC logs)
        drive.ata.smart_enable()?;

        // Enable VUC access
        drive.vuc_set_enabled(true)?;

        let results = vec![
            Self::check_native_info(drive),
            Self::check_read_sa(drive),
            Self::check_read_memory(drive),
            Self::check_iratemonk(drive),
        ]
        .into();

        // Disable VUC access
        drive.vuc_set_enabled(false)?;

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
            test_data::westerndigital_scorpioblack::IDENTIFY,
            test_data::westerndigital_bluemobile::IDENTIFY,
        ];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::seagate_momentus5::IDENTIFY,
            test_data::westerndigital_caviarse::IDENTIFY, // WDC MCU (WD70Cxx)
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
    fn parse_native_info() {
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::westerndigital_scorpioblack::NATIVE_INFO,
            test_data::westerndigital_bluemobile::NATIVE_INFO,
        ];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(NativeInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(NativeInfo::try_from(data).is_err());
        }
    }

    #[test]
    fn parse_file_header() {
        const DATA_VALID: &[&[u8]] = &[
            test_data::westerndigital_scorpioblack::FILE_1H,
            test_data::westerndigital_bluemobile::FILE_1H,
        ];
        const DATA_INVALID: &[&[u8; FileHeader::MAX_SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            let header = FileHeader::try_from(data).unwrap();
            let header_size = header.header_size as usize;
            assert!(header_size >= FileHeader::MIN_SIZE);
            assert!(header_size <= FileHeader::MAX_SIZE);
            let file_size = (header.sector_count as usize) * SECTOR_SIZE;
            assert_eq!(file_size, data.len());
        }

        for &data in DATA_INVALID {
            assert!(FileHeader::try_from(data.as_slice()).is_err());
        }
    }

    #[test]
    fn parse_file_directory() {
        const DATA_VALID: &[&[u8]] = &[
            test_data::westerndigital_scorpioblack::FILE_1H,
            test_data::westerndigital_bluemobile::FILE_1H,
        ];
        const DATA_INVALID: &[&[u8; SECTOR_SIZE * 10]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            let directory = FileDirectory::try_from(data).unwrap();
            assert!(!directory.entries.is_empty());
        }

        for &data in DATA_INVALID {
            assert!(FileDirectory::try_from(data.as_slice()).is_err());
        }
    }

    #[test]
    fn is_exception_vector() {
        const DATA_VALID: &[&[u8; SECTOR_SIZE]] = &[
            test_data::westerndigital_scorpioblack::EXCEPTION_VECTOR,
            test_data::westerndigital_bluemobile::EXCEPTION_VECTOR,
        ];
        const DATA_INVALID: &[&[u8; SECTOR_SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(Drive::is_exception_vector(data));
        }

        for &data in DATA_INVALID {
            assert!(!Drive::is_exception_vector(data));
        }
    }
}
