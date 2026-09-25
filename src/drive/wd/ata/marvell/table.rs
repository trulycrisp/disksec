//! VUC table structures.

use crate::protocol::ata::SECTOR_SIZE;

/// Table error.
#[derive(Debug)]
pub(super) enum Error {
    /// Invalid string.
    String(Box<[u8]>),
    /// Invalid format field.
    Format(u16),
    /// Invalid physical parameters drive type field.
    DriveType(u8),
    /// Invalid physical parameters flash type field.
    FlashType(u8),
    /// Invalid physical parameters PCBA type field.
    PcbaType(u8),
    /// Invalid physical parameters interface field.
    Interface(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(x) => write!(f, "invalid string {}", crate::output::format_bytes_hex(x)),
            Self::Format(x) => write!(f, "invalid table format {x}"),
            Self::DriveType(x) => write!(f, "invalid physical parameters drive type {x}"),
            Self::FlashType(x) => write!(f, "invalid physical parameters flash type {x}"),
            Self::PcbaType(x) => write!(f, "invalid physical parameters PCBA type {x}"),
            Self::Interface(x) => write!(f, "invalid physical parameters interface {x}"),
        }
    }
}

/// Parse string.
fn parse_string(data: &[u8]) -> Result<String, Error> {
    let string = str::from_utf8(data).map_err(|_| Error::String(data.into()))?;
    // Trim at null terminator, and trailing spaces
    let string = string.split('\0').next().unwrap_or("").trim_end();

    if !string
        .chars()
        .all(|x| x == ' ' || x.is_ascii_graphic())
    {
        return Err(Error::String(data.into()));
    }

    Ok(string.into())
}

/// Physical parameters drive type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum DriveType {
    /// Desktop 3.5 inch.
    Desktop3_5 = 0,
    /// Enterprise 3.5 inch.
    Enterprise3_5 = 1,
    /// Desktop 2.5 inch.
    Desktop2_5 = 2,
    /// Mobile 2.5 inch.
    Mobile2_5 = 3,
    /// Mobile 1.8 inch.
    Mobile1_8 = 4,
    /// Mobile 1.0 inch.
    Mobile1_0 = 5,
    /// Enterprise 2.5 inch.
    Enterprise2_5 = 6,
}

impl std::fmt::Display for DriveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveType::Desktop3_5 => write!(f, "desktop 3.5 inch"),
            DriveType::Enterprise3_5 => write!(f, "enterprise 3.5 inch"),
            DriveType::Desktop2_5 => write!(f, "desktop 2.5 inch"),
            DriveType::Mobile2_5 => write!(f, "mobile 2.5 inch"),
            DriveType::Mobile1_8 => write!(f, "mobile 1.8 inch"),
            DriveType::Mobile1_0 => write!(f, "mobile 1.0 inch"),
            DriveType::Enterprise2_5 => write!(f, "enterprise 2.5 inch"),
        }
    }
}

impl TryFrom<u8> for DriveType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[DriveType] = &[
            DriveType::Desktop3_5,
            DriveType::Enterprise3_5,
            DriveType::Desktop2_5,
            DriveType::Mobile2_5,
            DriveType::Mobile1_8,
            DriveType::Mobile1_0,
            DriveType::Enterprise2_5,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::DriveType(value))
    }
}

/// Physical parameters flash type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum FlashType {
    /// ROM (OTP).
    RomOtp = 1,
    /// ROM (custom OTP).
    RomCustomOtp = 2,
    /// Parallel flash.
    Parallel = 3,
    /// Custom flash.
    Custom = 4,
    /// Serial flash.
    Serial = 5,
    /// Manteca.
    Manteca = 6,
}

impl std::fmt::Display for FlashType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlashType::RomOtp => write!(f, "ROM (OTP)"),
            FlashType::RomCustomOtp => write!(f, "ROM (custom OTP)"),
            FlashType::Parallel => write!(f, "parallel flash"),
            FlashType::Custom => write!(f, "custom flash"),
            FlashType::Serial => write!(f, "serial flash"),
            FlashType::Manteca => write!(f, "manteca"),
        }
    }
}

impl TryFrom<u8> for FlashType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[FlashType] = &[
            FlashType::RomOtp,
            FlashType::RomCustomOtp,
            FlashType::Parallel,
            FlashType::Custom,
            FlashType::Serial,
            FlashType::Manteca,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::FlashType(value))
    }
}

/// Physical parameters interface.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum Interface {
    /// Unknown.
    Unknown = 0,
    /// PATA.
    Pata = 1,
    /// SATA.
    Sata = 2,
    /// USB.
    Usb = 3,
    /// Fibre Channel.
    Fc = 4,
    /// SAS.
    Sas = 5,
    /// CompactFlash.
    Cfa = 6,
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interface::Unknown => write!(f, "unknown"),
            Interface::Pata => write!(f, "PATA"),
            Interface::Sata => write!(f, "SATA"),
            Interface::Usb => write!(f, "USB"),
            Interface::Fc => write!(f, "Fibre Channel"),
            Interface::Sas => write!(f, "SAS"),
            Interface::Cfa => write!(f, "CompactFlash"),
        }
    }
}

impl TryFrom<u8> for Interface {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[Interface] = &[
            Interface::Unknown,
            Interface::Pata,
            Interface::Sata,
            Interface::Usb,
            Interface::Fc,
            Interface::Sas,
            Interface::Cfa,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::Interface(value))
    }
}

/// Physical parameters table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PhysicalParameters {
    /// Controller firmware version.
    pub(super) controller_firmware: String,
    /// Servo firmware version.
    pub(super) servo_firmware: String,
    /// Overlay firmware version.
    pub(super) overlay_firmware: String,
    /// Drive product ID.
    pub(super) product_id: u16,
    /// Physical platter count.
    pub(super) platter_count: u8,
    /// Physical head count.
    pub(super) physical_head_count: u8,
    /// Virtual head count, excludes depopulated.
    pub(super) virtual_head_count: u8,
    /// Populated head bitmap, bit set means head in use.
    pub(super) populated_heads: u16,
    /// Drive type and form factor.
    pub(super) drive_type: DriveType,
    /// Number of zones including SA.
    pub(super) zone_count: u8,
    /// Number of servo wedges in each track.
    pub(super) servo_wedges_per_track: u16,
    /// Number of negative cylinders reserved to the SA.
    pub(super) sa_cylinder_count: u16,
    /// Spindle rotations per minute.
    pub(super) rpm: u16,
    /// Number of cylinders in the user area, always zero on newer drives.
    pub(super) user_cylinder_count: u32,
    /// Cylinder skew in servo wedges.
    pub(super) cylinder_skew: u8,
    /// Head skew in servo wedges.
    pub(super) head_skew: u8,
    /// Cluster skew in servo wedges.
    pub(super) cluster_skew: u8,
    /// Flash storage type.
    pub(super) flash_type: FlashType,
    /// PCB assembly is not Entek.
    pub(super) pcba_non_entek: bool,
    /// Drive bus interface.
    pub(super) interface: Interface,
    /// Controller vendor ID.
    pub(super) controller_vendor: u16,
    /// Controller revision ID.
    pub(super) controller_revision: u16,
    /// Microprocessor vendor ID.
    pub(super) microprocessor_vendor: u16,
    /// Microprocessor revision ID.
    pub(super) microprocessor_revision: u16,
    /// Read channel vendor ID.
    pub(super) read_channel_vendor: u16,
    /// Read channel revision ID.
    pub(super) read_channel_revision: u16,
    /// Preamplifier vendor ID.
    pub(super) preamp_vendor: u16,
    /// Preamplifier revision ID.
    pub(super) preamp_revision: u16,
    /// Power integrated-circuit vendor ID.
    pub(super) power_ic_vendor: u16,
    /// Power integrated-circuit revision ID.
    pub(super) power_ic_revision: u16,
    /// Independently corrected ECC interleaves per sector codeword.
    pub(super) otf_interleaves: u8,
    /// Bytes correctable on the fly, per interleave.
    pub(super) otf_correction_bytes: u8,
    /// ECC bytes correctable by a burst correction.
    pub(super) burst_correction_bytes: u16,
    /// Error correction  redundancy bytes per sector.
    pub(super) ecc_bytes: u8,
    /// Largest supported VUC data transfer in sectors.
    pub(super) vuc_max_transfer: u16,
    /// Surface format revision.
    pub(super) format_revision: String,
    /// Read-channel firmware version.
    pub(super) read_channel_firmware: String,
    /// VUC action-code 55 auto configuration supported.
    pub(super) vuc_ac55_auto_config: bool,
    /// VUC action-code 55 dynamic parameters supported.
    pub(super) vuc_ac55_dynamic_params: bool,
    /// Data Lifeguard version 2 supported.
    pub(super) dlg_2: bool,
    /// Data Lifeguard version 3 supported.
    pub(super) dlg_3: bool,
    /// Data Lifeguard extended supported.
    pub(super) dlg_ext: bool,
    /// Data Lifeguard write supported.
    pub(super) dlg_write: bool,
    /// Reserved Area Robustness supported.
    pub(super) rar: bool,
    /// Data Lifeguard version 3+ supported.
    pub(super) dlg_3plus: bool,
    /// Data Lifeguard version 4 supported.
    pub(super) dlg_4: bool,
    /// Advanced format alignment offset, unknown specific meaning.
    pub(super) alignment_offset: u8,
    /// User area sector size.
    pub(super) sector_size: u32,
    /// PATA jumper block master.
    pub(super) jumper_master: bool,
    /// PATA jumper block slave.
    pub(super) jumper_slave: bool,
    /// PATA jumper block cable select.
    pub(super) jumper_cable_select: bool,
    /// Sectors remapped to spares, grown defects.
    pub(super) relocation_count: u16,
    /// Defect entries logged during the factory surface scan.
    pub(super) tare_count: u16,
    /// Defect entries logged after the drive left the factory.
    pub(super) field_tare_count: u16,
    /// Relocations remaining before the spare pool is exhausted.
    pub(super) remaining_relocations: u16,
    /// Spare sectors already used.
    pub(super) consumed_spares: u16,
    /// Last logical block address of the spare pool.
    pub(super) last_spare_lba: u64,
    /// Selected radial track density, in thousands of tracks per inch.
    pub(super) tracks_per_inch: u16,
    /// Areal density per platter, in units of 100 MB.
    pub(super) area_density: u16,
    /// Drive format version ID, zero is the primary format.
    pub(super) format_type: u8,
    /// Controller family ID, distinct from the vendor and revision pair.
    pub(super) controller_id: u16,
    /// PCB assembly revision.
    pub(super) pcba_revision: u8,
    /// Enhanced security support.
    pub(super) enhanced_security: bool,
    /// Zone cache priority pool support.
    pub(super) zone_cache_priority_pool: bool,
    /// ECC symbols correctable by one interleave of on-the-fly correction.
    pub(super) otf_correction_symbols: u16,
    /// ECC symbols correctable by a burst correction.
    pub(super) burst_correction_symbols: u16,
    /// Bits between the first symbol start and the first bit in a sector.
    pub(super) symbol_bit_offset: u8,
    /// Flash capacity in bytes.
    pub(super) flash_size: u32,
    /// Flash programming protocol version, unknown specific meaning.
    pub(super) flash_protocol_version: u8,
    /// Flash erase block size, in 2 KiB units.
    pub(super) flash_sector_size: u8,
    /// Head class for defect and reliability tables, unknown specific meaning.
    pub(super) head_format_class: u8,
    /// Head-disk assembly type unknown specific meaning.
    pub(super) hda_type: u8,
    /// SATA link idle before entering the slumber state, in milliseconds.
    pub(super) slumber_timer: u16,
    /// SATA link idle before entering the partial state, in milliseconds.
    pub(super) partial_timer: u16,
    /// Read time-limited error recovery ceiling in milliseconds, zero is off.
    pub(super) read_tler: u16,
    /// Write time-limited error recovery ceiling in milliseconds, zero is off.
    pub(super) write_tler: u16,
    /// Number of 512-byte buffers allocated to cache.
    pub(super) cache_buffers: u32,
    /// Physical DRAM size in megabytes.
    pub(super) dram_size: u16,
    /// Configuration management cache family.
    pub(super) uccm_cache_family: String,
    /// Configuration management dash code, the model number suffix.
    pub(super) uccm_dash_code: String,
    /// Configuration management configuration version.
    pub(super) uccm_config_version: (u16, u16),
    /// Configuration management firmware structure version.
    pub(super) uccm_firmware_struct_version: (u16, u16),
    /// Configuration management dash code was modified, `None` if undefined.
    pub(super) uccm_modified: Option<bool>,
    /// Configuration management customer configuration code.
    pub(super) uccm_customer_code: u32,
    /// Drive model.
    pub(super) model: String,
    /// Drive Configuration Matrix.
    pub(super) drive_config_matrix: String,
    /// Manufacture date, the original drive build date.
    pub(super) manufacture_date: String,
    /// Remanufacture date, empty if the drive was never reconfigured.
    pub(super) remanufacture_date: String,
    /// Primary defect list hash index size in bits.
    pub(super) pdlist_hash_size_bits: u8,
    /// Drive serial.
    pub(super) serial: String,
    /// Global product ID.
    pub(super) global_product_id: u16,
    /// Reference crystal frequency in megahertz.
    pub(super) crystal_frequency: u16,
    /// Maximum head count the platform supports.
    pub(super) max_head_count: u16,
    /// First user area Absolute Block Address.
    pub(super) first_user_aba: u64,
    /// Flash vendor ID.
    pub(super) flash_vendor: u32,
    /// Flash device ID.
    pub(super) flash_device: u32,
    /// Zone cache priority pool depth.
    pub(super) zone_cache_pool_depth: u16,
}

impl PhysicalParameters {
    /// Size in bytes.
    const SIZE: usize = SECTOR_SIZE;
}

impl TryFrom<&[u8; Self::SIZE]> for PhysicalParameters {
    type Error = Error;

    fn try_from(value: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        const FORMAT_VERSION: u16 = 2;
        const PRODUCT_ID_EXTENDED: u8 = 0xFE;
        const FLASH_SIZE_UNIT: u32 = 16_384;

        let parse_string_reversed =
            |data: &[u8]| parse_string(&data.iter().rev().copied().collect::<Box<[u8]>>());

        let format_version = u16::from_le_bytes([value[0], value[1]]);
        if format_version != FORMAT_VERSION {
            return Err(Error::Format(format_version));
        }

        let controller_firmware = parse_string(&value[2..10])?;
        let servo_firmware = parse_string(&value[10..18])?;
        let overlay_firmware = parse_string(&value[18..26])?;

        let product_id = match value[26] {
            PRODUCT_ID_EXTENDED => u16::from_le_bytes([value[364], value[365]]),
            x => u16::from(x),
        };

        let platter_count = value[28];
        let physical_head_count = value[29];
        let virtual_head_count = value[30];

        let populated_heads = match value[31] {
            x if x.count_ones() == virtual_head_count.into() => u16::from(x),
            _ => u16::from_le_bytes([value[370], value[371]]),
        };

        let drive_type = value[32].try_into()?;
        let zone_count = value[33];
        let servo_wedges_per_track = u16::from_le_bytes([value[34], value[35]]);
        let sa_cylinder_count = u16::from_le_bytes([value[36], value[37]]);
        let rpm = u16::from_le_bytes([value[38], value[39]]);
        let user_cylinder_count = u32::from_le_bytes([value[40], value[41], value[42], value[43]]);
        let cylinder_skew = value[44];
        let head_skew = value[45];
        let cluster_skew = value[46];
        let flash_type = value[47].try_into()?;
        let pcba_non_entek = bool::try_from(value[48]).or(Err(Error::PcbaType(value[48])))?;
        let interface = value[49].try_into()?;
        let controller_vendor = u16::from_le_bytes([value[50], value[51]]);
        let controller_revision = u16::from_le_bytes([value[52], value[53]]);
        let microprocessor_vendor = u16::from_le_bytes([value[54], value[55]]);
        let microprocessor_revision = u16::from_le_bytes([value[56], value[57]]);
        let read_channel_vendor = u16::from_le_bytes([value[58], value[59]]);
        let read_channel_revision = u16::from_le_bytes([value[60], value[61]]);
        let preamp_vendor = u16::from_le_bytes([value[62], value[63]]);
        let preamp_revision = u16::from_le_bytes([value[64], value[65]]);
        let power_ic_vendor = u16::from_le_bytes([value[66], value[67]]);
        let power_ic_revision = u16::from_le_bytes([value[68], value[69]]);
        let otf_interleaves = value[70];
        let otf_correction_bytes = value[71];
        let burst_correction_bytes = u16::from_le_bytes([value[72], value[73]]);
        let ecc_bytes = value[74];
        let vuc_max_transfer = u16::from_le_bytes([value[76], value[77]]);
        let format_revision = parse_string(&value[78..86])?;
        let read_channel_firmware = parse_string(&value[86..94])?;

        let feature_flags = u32::from_le_bytes([value[94], value[95], value[96], value[97]]);
        let vuc_ac55_auto_config = feature_flags & (1 << 5) != 0;
        let vuc_ac55_dynamic_params = feature_flags & (1 << 20) != 0;
        let dlg_2 = feature_flags & (1 << 22) != 0;
        let dlg_3 = feature_flags & (1 << 23) != 0;
        let dlg_ext = feature_flags & (1 << 24) != 0;
        let dlg_write = feature_flags & (1 << 25) != 0;
        let rar = feature_flags & (1 << 26) != 0;
        let dlg_3plus = feature_flags & (1 << 27) != 0;
        let dlg_4 = feature_flags & (1 << 28) != 0;
        let alignment_offset = ((feature_flags >> 31) & 1) as u8;

        let feature_flags2 = u32::from_le_bytes([value[98], value[99], value[100], value[101]]);
        let sector_size = (u32::try_from(SECTOR_SIZE).unwrap()) << (feature_flags2 & 0b111);

        let jumper_setting = u16::from_le_bytes([value[118], value[119]]);
        let jumper_master = jumper_setting & (1 << 0) != 0;
        let jumper_slave = jumper_setting & (1 << 1) != 0;
        let jumper_cable_select = jumper_setting & (1 << 2) != 0;

        let relocation_count = u16::from_le_bytes([value[120], value[121]]);
        let tare_count = u16::from_le_bytes([value[122], value[123]]);
        let field_tare_count = u16::from_le_bytes([value[124], value[125]]);
        let remaining_relocations = u16::from_le_bytes([value[126], value[127]]);
        let consumed_spares = u16::from_le_bytes([value[128], value[129]]);

        let last_spare_lba = u64::from_le_bytes([
            value[132], value[133], value[134], value[135], value[130], value[131], 0, 0,
        ]);

        let tracks_per_inch = u16::from_le_bytes([value[136], value[137]]);
        let area_density = u16::from_le_bytes([value[138], value[139]]);
        let format_type = value[140];
        let controller_id = u16::from_le_bytes([value[142], value[143]]);
        let pcba_revision = value[144];

        let feature_support = u64::from_le_bytes([
            value[148], value[149], value[150], value[151], value[152], value[153], value[154],
            value[155],
        ]);

        let enhanced_security = feature_support & (1 << 0) != 0;
        let zone_cache_priority_pool = feature_support & (1 << 10) != 0;

        let otf_correction_symbols = u16::from_le_bytes([value[156], value[157]]);
        let burst_correction_symbols = u16::from_le_bytes([value[158], value[159]]);
        let symbol_bit_offset = value[160];
        let flash_size = u32::from(value[164]) * FLASH_SIZE_UNIT;
        let flash_protocol_version = value[165];
        let flash_sector_size = value[166];
        let head_format_class = value[167];
        let hda_type = value[168];
        let slumber_timer = u16::from_le_bytes([value[170], value[171]]);
        let partial_timer = u16::from_le_bytes([value[172], value[173]]);
        let read_tler = u16::from_le_bytes([value[174], value[175]]);
        let write_tler = u16::from_le_bytes([value[176], value[177]]);
        let cache_buffers = u32::from_le_bytes([value[178], value[179], value[180], value[181]]);
        let dram_size = u16::from_le_bytes([value[182], value[183]]);

        let uccm_cache_family = parse_string_reversed(&value[184..188])?;
        let uccm_dash_code = parse_string_reversed(&value[188..192])?;

        let uccm_config_version = (
            u16::from_le_bytes([value[192], value[193]]),
            u16::from_le_bytes([value[194], value[195]]),
        );

        let uccm_firmware_struct_version = (
            u16::from_le_bytes([value[196], value[197]]),
            u16::from_le_bytes([value[198], value[199]]),
        );

        let uccm_modified = (value[200] & (1 << 7) == 0).then(|| value[200] & 1 != 0);
        let uccm_customer_code =
            u32::from_le_bytes([value[202], value[203], value[204], value[205]]);
        let model = parse_string(&value[210..234])?;
        let drive_config_matrix = parse_string(&value[256..292])?;
        let manufacture_date = parse_string(&value[292..304])?;
        let remanufacture_date = parse_string(&value[304..316])?;
        let pdlist_hash_size_bits = value[316];
        let serial = parse_string(&value[318..338])?;
        let global_product_id = u16::from_le_bytes([value[366], value[367]]);
        let crystal_frequency = u16::from_le_bytes([value[368], value[369]]);
        let max_head_count = u16::from_le_bytes([value[372], value[373]]);

        let first_user_aba = u64::from_le_bytes([
            value[376], value[377], value[378], value[379], value[380], value[381], value[382],
            value[383],
        ]);

        let flash_vendor = u32::from_le_bytes([value[386], value[387], value[388], value[389]]);
        let flash_device = u32::from_le_bytes([value[390], value[391], value[392], value[393]]);
        let zone_cache_pool_depth = u16::from_le_bytes([value[394], value[395]]);

        Ok(Self {
            controller_firmware,
            servo_firmware,
            overlay_firmware,
            product_id,
            platter_count,
            physical_head_count,
            virtual_head_count,
            populated_heads,
            drive_type,
            zone_count,
            servo_wedges_per_track,
            sa_cylinder_count,
            rpm,
            user_cylinder_count,
            cylinder_skew,
            head_skew,
            cluster_skew,
            flash_type,
            pcba_non_entek,
            interface,
            controller_vendor,
            controller_revision,
            microprocessor_vendor,
            microprocessor_revision,
            read_channel_vendor,
            read_channel_revision,
            preamp_vendor,
            preamp_revision,
            power_ic_vendor,
            power_ic_revision,
            otf_interleaves,
            otf_correction_bytes,
            burst_correction_bytes,
            ecc_bytes,
            vuc_max_transfer,
            format_revision,
            read_channel_firmware,
            vuc_ac55_auto_config,
            vuc_ac55_dynamic_params,
            dlg_2,
            dlg_3,
            dlg_ext,
            dlg_write,
            rar,
            dlg_3plus,
            dlg_4,
            alignment_offset,
            sector_size,
            jumper_master,
            jumper_slave,
            jumper_cable_select,
            relocation_count,
            tare_count,
            field_tare_count,
            remaining_relocations,
            consumed_spares,
            last_spare_lba,
            tracks_per_inch,
            area_density,
            format_type,
            controller_id,
            pcba_revision,
            enhanced_security,
            zone_cache_priority_pool,
            otf_correction_symbols,
            burst_correction_symbols,
            symbol_bit_offset,
            flash_size,
            flash_protocol_version,
            flash_sector_size,
            head_format_class,
            hda_type,
            slumber_timer,
            partial_timer,
            read_tler,
            write_tler,
            cache_buffers,
            dram_size,
            uccm_cache_family,
            uccm_dash_code,
            uccm_config_version,
            uccm_firmware_struct_version,
            uccm_modified,
            uccm_customer_code,
            model,
            drive_config_matrix,
            manufacture_date,
            remanufacture_date,
            pdlist_hash_size_bits,
            serial,
            global_product_id,
            crystal_frequency,
            max_head_count,
            first_user_aba,
            flash_vendor,
            flash_device,
            zone_cache_pool_depth,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_physical_parameters() {
        const DATA_VALID: &[&[u8; PhysicalParameters::SIZE]] = &[
            test_data::westerndigital_scorpioblack::PHYSICAL_PARAMETERS,
            test_data::westerndigital_bluemobile::PHYSICAL_PARAMETERS,
        ];
        const DATA_INVALID: &[&[u8; PhysicalParameters::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(PhysicalParameters::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(PhysicalParameters::try_from(data).is_err());
        }
    }
}
