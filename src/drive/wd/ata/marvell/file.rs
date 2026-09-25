//! SA/flash file definitions.

use crate::protocol::ata::SECTOR_SIZE;

/// SA error.
#[derive(Debug)]
pub(super) enum Error {
    /// Invalid file header type field.
    InvalidFileType(u8),
    /// File header truncated (too short).
    FileHeaderTruncated,
    /// Invalid file header magic.
    InvalidFileHeaderMagic,
    /// Invalid file header size field.
    InvalidFileHeaderSize(u16),
    /// Invalid file header sector count field.
    InvalidFileSectorCount(u16),
    /// Invalid file header version field.
    InvalidFileHeaderVersion,
    /// Directory truncated.
    DirectoryTruncated,
    /// Invalid directory version.
    InvalidDirectoryVersion(String),
    /// Duplicate directory entry
    DuplicateDirectoryEntry(u16),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFileType(x) => write!(f, "invalid file type {x}"),
            Self::FileHeaderTruncated => write!(f, "file header truncated"),
            Self::InvalidFileHeaderMagic => write!(f, "invalid file header magic"),
            Self::InvalidFileHeaderSize(x) => write!(f, "invalid file header size {x}"),
            Self::InvalidFileSectorCount(x) => write!(f, "invalid file header sector count {x}"),
            Self::InvalidFileHeaderVersion => write!(f, "invalid file header version"),
            Self::DirectoryTruncated => write!(f, "directory truncated"),
            Self::InvalidDirectoryVersion(x) => write!(f, "invalid directory version {x}"),
            Self::DuplicateDirectoryEntry(x) => write!(f, "duplicate directory entry {x}"),
        }
    }
}

/// File ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FileId {
    /// System area directory.
    SaDirectory = 0x1,
    /// Flash directory.
    FlashDirectory = 0xB,
    /// Flash directory second copy.
    FlashDirectoryExt = 0x20B,
}

/// File header 2 type field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FileType {
    /// Generic.
    Generic = 1,
    /// Process Self-Test.
    Pst = 2,
    /// Firmware overlay,
    Overlay = 3,
    /// Flash resident,
    Flash = 4,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic => write!(f, "generic"),
            Self::Pst => write!(f, "process self-test"),
            Self::Overlay => write!(f, "overlay"),
            Self::Flash => write!(f, "flash"),
        }
    }
}

impl TryFrom<u8> for FileType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[FileType] = &[
            FileType::Generic,
            FileType::Pst,
            FileType::Overlay,
            FileType::Flash,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidFileType(value))
    }
}

/// Header of file, version 2 (ROYL) variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FileHeader {
    /// File type.
    pub(super) file_type: FileType,
    /// Sector count high bits, only used in newer headers.
    pub(super) sector_count_high: u8,
    /// File ID.
    pub(super) id: u16,
    /// Total file size in sectors, including header.
    pub(super) sector_count: u16,
    /// Checksum value.
    pub(super) checksum: u32,
    /// File data format version/revision.
    pub(super) version: String,
    /// File month.
    pub(super) month: u8,
    /// File day.
    pub(super) day: u8,
    /// File year.
    pub(super) year: u8,
    /// File-specific tag value.
    pub(super) tag: u8,
    /// Additional file-specific data.
    pub(super) additional: Box<[u8]>,
}

impl FileHeader {
    /// Maximum size in bytes.
    pub(super) const MAX_SIZE: usize = SECTOR_SIZE;
    /// Minimum size in bytes.
    const MIN_SIZE: usize = 28;

    /// Get header size in bytes.
    fn size(&self) -> usize {
        Self::MIN_SIZE + self.additional.len()
    }
}

impl TryFrom<&[u8]> for FileHeader {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        const MAGIC: &[u8] = b"ROYL";

        if value.len() < Self::MIN_SIZE {
            return Err(Error::FileHeaderTruncated);
        }

        if !value.starts_with(MAGIC) {
            return Err(Error::InvalidFileHeaderMagic);
        }

        let file_type = FileType::try_from(value[4])?;

        let sector_count_high = match value[5] {
            x if x & (1 << 7) != 0 => x & 0b111_1111,
            _ => 0,
        };

        let header_size = u16::from_le_bytes([value[6], value[7]]);
        if !(Self::MIN_SIZE..=Self::MAX_SIZE.min(value.len())).contains(&(header_size as usize)) {
            return Err(Error::InvalidFileHeaderSize(header_size));
        }

        let id = u16::from_le_bytes([value[8], value[9]]);

        let sector_count = u16::from_le_bytes([value[10], value[11]]);
        if sector_count == 0 {
            return Err(Error::InvalidFileSectorCount(sector_count));
        }

        let checksum = u32::from_le_bytes([value[12], value[13], value[14], value[15]]);

        let version = std::str::from_utf8(&value[16..24])
            .or(Err(Error::InvalidFileHeaderVersion))?
            .trim_end_matches('\0')
            .into();

        let month = value[24];
        let day = value[25];
        let year = value[26];
        let tag = value[27];
        let additional = value[Self::MIN_SIZE..header_size as usize].into();

        Ok(Self {
            file_type,
            sector_count_high,
            id,
            sector_count,
            checksum,
            version,
            month,
            day,
            year,
            tag,
            additional,
        })
    }
}

/// File.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct File {
    /// File header.
    pub(super) header: FileHeader,
    /// File data.
    pub(super) data: Box<[u8]>,
}

impl TryFrom<&[u8]> for File {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let header = FileHeader::try_from(value)?;
        let data = (value[header.size()..]).into();

        Ok(Self { header, data })
    }
}

/// File attributes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FileAttributes {
    /// IF file exists in each region index.
    pub(super) regions: [bool; 8],
    /// Flash resident file.
    pub(super) flash: bool,
    /// File is intended to be read-only.
    pub(super) read_only: bool,
    /// Validate file checksum.
    pub(super) checksum: bool,
    /// File has standard file header.
    pub(super) header: bool,
    /// File is a placeholder without content.
    pub(super) placeholder: bool,
    /// File is always loaded in memory.
    pub(super) static_file: bool,
    /// Process Test Module.
    pub(super) ptm: bool,
    /// Super Process Test Module.
    pub(super) super_ptm: bool,
    /// Firmware overlay loaded in transient cache memory.
    pub(super) transient: bool,
    /// Unknown, described as "file create".
    pub(super) create: bool,
    /// Preserve file on firmware update.
    pub(super) preserve: bool,
    /// Firmware code to be overwrote through firmware update.
    pub(super) field_code: bool,
    /// File created from the region end instead of start.
    pub(super) create_from_end: bool,
}

impl From<u32> for FileAttributes {
    fn from(value: u32) -> Self {
        let regions = std::array::from_fn(|i| value & (1 << i) != 0);
        let flash = value & (1 << 8) != 0;
        let read_only = value & (1 << 10) != 0;
        let checksum = value & (1 << 11) != 0;
        let header = value & (1 << 12) != 0;
        let placeholder = value & (1 << 13) != 0;
        let static_file = value & (1 << 14) != 0;
        let ptm = value & (1 << 15) != 0;
        let super_ptm = value & (1 << 16) != 0;
        let transient = value & (1 << 17) != 0;
        let create = value & (1 << 18) != 0;
        let preserve = value & (1 << 19) != 0;
        let field_code = value & (1 << 20) != 0;
        let create_from_end = value & (1 << 21) != 0;

        Self {
            regions,
            flash,
            read_only,
            checksum,
            header,
            placeholder,
            static_file,
            ptm,
            super_ptm,
            transient,
            create,
            preserve,
            field_code,
            create_from_end,
        }
    }
}

/// Directory entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct DirectoryEntry {
    /// Number of copies accross regions.
    pub(super) copy_count: u8,
    /// File ID.
    pub(super) id: u16,
    /// File size, 512-byte sectors for SA, bytes for flash.
    pub(super) size: u32,
    /// File attributes.
    pub(super) attributes: FileAttributes,
    /// File address indexed by copy, Reserved LBA for SA, bytes for flash.
    pub(super) address: Box<[u32]>,
}

impl DirectoryEntry {
    /// Minumum entry size in bytes.
    const MIN_SIZE: usize = 14;

    /// Is entry valid or an empty placeholder.
    pub(super) fn is_valid(&self) -> bool {
        self.copy_count > 0
    }

    /// Get size of file in bytes.
    pub(super) fn size(&self) -> u64 {
        let mut size = self.size.into();

        if !self.attributes.flash {
            size *= SECTOR_SIZE as u64;
        }

        size
    }
}

impl TryFrom<&[u8]> for DirectoryEntry {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < Self::MIN_SIZE {
            return Err(Error::DirectoryTruncated);
        }

        let copy_count = value[1];
        let id: u16 = u16::from_le_bytes([value[2], value[3]]);

        let (size_size, size) = match value.len() % size_of::<u32>() {
            0 => (
                size_of::<u32>(),
                u32::from_le_bytes([value[4], value[5], value[6], value[7]]),
            ),
            x if x == size_of::<u16>() => (
                size_of::<u16>(),
                u32::from(u16::from_le_bytes([value[4], value[5]])),
            ),
            _ => {
                return Err(Error::DirectoryTruncated);
            },
        };

        let attributes = FileAttributes::from(u32::from_le_bytes(std::array::from_fn(|i| {
            value[4 + size_size + i]
        })));

        let (address_chunks, _) = value[8 + size_size..].as_chunks::<{ size_of::<u32>() }>();
        let address = address_chunks
            .iter()
            .copied()
            .map(u32::from_le_bytes)
            .collect();

        Ok(Self {
            copy_count,
            id,
            size,
            attributes,
            address,
        })
    }
}

/// Directory of files.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Directory {
    /// Directory entries.
    pub(super) entries: Box<[DirectoryEntry]>,
}

impl Directory {
    /// ROYL-ABA SA/flash directory version.
    pub(super) const VERSION_ROYL_ABA: &str = "00020000";
    /// ROYL-ABA-20B flash directory version.
    pub(super) const VERSION_ROYL_ABA_20B_FLASH: &str = "00030000";

    /// Get valid entries.
    pub(super) fn valid(&self) -> impl Iterator<Item = &DirectoryEntry> {
        self.entries.iter().filter(|x| x.is_valid())
    }
}

impl TryFrom<&File> for Directory {
    type Error = Error;

    fn try_from(file: &File) -> Result<Self, Self::Error> {
        const VERSIONS: &[&str] = &[
            Directory::VERSION_ROYL_ABA,
            Directory::VERSION_ROYL_ABA_20B_FLASH,
        ];
        const MIN_SIZE: usize = 1 + DirectoryEntry::MIN_SIZE; // 8-bit entry count + entry

        if !VERSIONS.contains(&file.header.version.as_str()) {
            return Err(Error::InvalidDirectoryVersion(file.header.version.clone()));
        }

        if file.data.len() < MIN_SIZE {
            return Err(Error::DirectoryTruncated);
        }

        // SA directory has a 16-bit entry count, flash has 8-bit
        let (entry_count, entries_offset) = if file.header.file_type == FileType::Flash {
            (usize::from(file.data[0]), size_of::<u8>())
        } else {
            (
                usize::from(u16::from_le_bytes([file.data[0], file.data[1]])),
                size_of::<u16>(),
            )
        };

        let entry_size = usize::from(file.data[entries_offset]);

        if entry_size < DirectoryEntry::MIN_SIZE {
            return Err(Error::DirectoryTruncated);
        }

        let entries = file
            .data
            .get(entries_offset..entries_offset + entry_count * entry_size)
            .ok_or(Error::DirectoryTruncated)?
            .chunks_exact(entry_size)
            .map(DirectoryEntry::try_from)
            .collect::<Result<Box<_>, _>>()?;

        // Check for duplicate entries
        let mut seen = std::collections::HashSet::new();
        if let Some(duplicate) = entries
            .iter()
            .filter(|x| x.is_valid())
            .find(|x| !seen.insert(x.id))
        {
            return Err(Error::DuplicateDirectoryEntry(duplicate.id));
        }

        Ok(Self { entries })
    }
}

impl TryFrom<&[u8]> for Directory {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        (&File::try_from(value)?).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_file_header() {
        const DATA_VALID: &[&[u8]] = &[
            test_data::westerndigital_scorpioblack::FILE_1H,
            test_data::westerndigital_bluemobile::FILE_1H,
        ];
        const DATA_INVALID: &[&[u8; FileHeader::MAX_SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            let header = FileHeader::try_from(data).unwrap();
            let file_size = (header.sector_count as usize) * SECTOR_SIZE;
            assert_eq!(file_size, data.len());
        }

        for &data in DATA_INVALID {
            assert!(FileHeader::try_from(data.as_slice()).is_err());
        }
    }

    #[test]
    fn parse_directory() {
        const DATA_VALID: &[&[u8]] = &[
            test_data::westerndigital_scorpioblack::FILE_1H,
            test_data::westerndigital_bluemobile::FILE_1H,
        ];
        const DATA_INVALID: &[&[u8; SECTOR_SIZE * 10]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            let directory = Directory::try_from(data).unwrap();
            assert!(!directory.entries.is_empty());
        }

        for &data in DATA_INVALID {
            assert!(Directory::try_from(data.as_slice()).is_err());
        }
    }
}
