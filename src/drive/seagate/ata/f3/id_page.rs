//! ID Page parsing.

use std::fmt::Write;

/// Page size in bytes.
pub(super) const SIZE: usize = crate::protocol::ata::SECTOR_SIZE;
/// Size of timestamp fields in bytes.
const TIMESTAMP_SIZE: usize = 14;

/// ID page error.
#[derive(Debug)]
pub(super) enum Error {
    /// Invalid string.
    String(Box<[u8]>),
    /// Invalid timestamp.
    Timestamp(Box<[u8]>),
    /// Invalid or unsupported version.
    Version(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(x) => write!(f, "invalid string {}", crate::output::format_bytes_hex(x)),
            Self::Timestamp(x) => {
                write!(
                    f,
                    "invalid timestamp {}",
                    crate::output::format_bytes_hex(x)
                )
            },
            Self::Version(x) => write!(f, "invalid version {x}"),
        }
    }
}

/// Parse string.
fn parse_string(data: &[u8]) -> Result<&str, Error> {
    let string = str::from_utf8(data).map_err(|_| Error::String(data.into()))?;
    // Trim at null terminator, and trailing spaces
    let string = string.split('\0').next().unwrap_or("").trim_end();

    if string.is_empty() {
        return Err(Error::String(data.into()));
    }

    if !string
        .chars()
        .all(|x| x == ' ' || x.is_ascii_graphic())
    {
        return Err(Error::String(data.into()));
    }

    Ok(string)
}

/// Parse optional string.
fn parse_option_string(data: &[u8]) -> Result<Option<&str>, Error> {
    if data.iter().all(|&x| x == 0) {
        return Ok(None);
    }

    let string = parse_string(data)?;

    if string.chars().all(|x| x == '-') {
        return Ok(None);
    }

    Ok(Some(string))
}

/// Parse timestamp string.
fn parse_timestamp(data: &[u8; TIMESTAMP_SIZE]) -> Result<String, Error> {
    const DATE_SIZE: usize = 8;

    let date = parse_string(&data[..DATE_SIZE])?;
    let time = parse_option_string(&data[DATE_SIZE..])?;

    if date.len() != DATE_SIZE
        || !date.chars().all(|x| char::is_ascii_digit(&x))
        || time.is_some_and(|x| {
            x.len() != TIMESTAMP_SIZE - DATE_SIZE || !x.chars().all(|x| char::is_ascii_digit(&x))
        })
    {
        return Err(Error::Timestamp(data.as_ref().into()));
    }

    let mut timestamp = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);

    if let Some(time) = time {
        write!(timestamp, " {}:{}:{}", &time[..2], &time[2..4], &time[4..]).unwrap();
    }

    Ok(timestamp)
}

/// ID Page 0, basic info and page directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Page0 {
    /// Firmware version.
    pub(super) firmware_version: String,
    /// Controller firmware revision.
    pub(super) cfw_version: String,
    /// Firmware changelist number.
    pub(super) changelist: String,
    /// Directory of available ID pages and their version.
    pub(super) directory: [Option<u8>; 48],
    /// Firmware build site location.
    pub(super) firmware_build_site: String,
    /// Firmware build timestamp.
    pub(super) firmware_timestamp: String,
}

impl TryFrom<&[u8; SIZE]> for Page0 {
    type Error = Error;

    fn try_from(value: &[u8; SIZE]) -> Result<Self, Self::Error> {
        let firmware_version = parse_string(&value[..8])?.into();
        let cfw_version = parse_string(&value[8..16])?.into();
        let changelist = parse_string(&value[16..24])?.into();
        let directory = std::array::from_fn(|i| Some(value[32 + i]).filter(|&y| y != 0));
        let firmware_build_site = parse_string(&value[128..144])?.into();

        let firmware_timestamp =
            parse_timestamp(&value[144..144 + TIMESTAMP_SIZE].try_into().unwrap())?;

        Ok(Self {
            firmware_version,
            cfw_version,
            changelist,
            directory,
            firmware_build_site,
            firmware_timestamp,
        })
    }
}

/// ID Page 11, firmware information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Page11 {
    /// Firmware package version.
    pub(super) package_version: String,
    /// Firmware package part number.
    pub(super) package_part: Option<String>,
    /// Firmware package Global ID (a.k.a Builder ID).
    pub(super) package_global_id: String,
    /// Firmware package build timestamp.
    pub(super) package_timestamp: String,
    /// Firmware package controller firmware version.
    pub(super) package_cfw_version: String,
    /// Firmware package servo firmware version.
    pub(super) package_sfw_version: [Option<String>; 4],
    /// Controller firmware build timestamp.
    pub(super) cfw_timestamp: String,
    /// Active servo firmware version.
    pub(super) sfw_version: String,
    /// TCG initialization vector version.
    pub(super) tcg_iv_version: Option<String>,
}

impl Page11 {
    /// Earlier version number.
    const VERSION_1: u8 = 0x10;
    /// Later version number.
    const VERSION_2: u8 = 0x11;

    /// Parse page from data.
    pub(super) fn parse(value: &[u8; SIZE], version: u8) -> Result<Self, Error> {
        let package_cfw_version_end = match version {
            Self::VERSION_1 => 160,
            Self::VERSION_2 => 192,
            x => return Err(Error::Version(x)),
        };

        let package_version = parse_string(&value[..64])?.into();
        let package_part = parse_option_string(&value[64..96])?.map(str::to_string);
        let package_global_id = parse_string(&value[96..112])?.into();
        let package_timestamp =
            parse_timestamp(&value[112..112 + TIMESTAMP_SIZE].try_into().unwrap())?;
        let package_cfw_version = parse_string(&value[128..package_cfw_version_end])?.into();

        let parse_package_sfw_version = |i: usize| {
            let offset = package_cfw_version_end + (16 * i);
            parse_option_string(&value[offset..offset + 16]).map(|x| x.map(str::to_string))
        };
        let package_sfw_version = [
            parse_package_sfw_version(0)?,
            parse_package_sfw_version(1)?,
            parse_package_sfw_version(2)?,
            parse_package_sfw_version(3)?,
        ];

        let cfw_timestamp = parse_timestamp(&value[448..448 + TIMESTAMP_SIZE].try_into().unwrap())?;
        let sfw_version = parse_string(&value[464..480])?.into();
        let tcg_iv_version = parse_option_string(&value[496..508])?.map(str::to_string);

        Ok(Self {
            package_version,
            package_part,
            package_global_id,
            package_timestamp,
            package_cfw_version,
            package_sfw_version,
            cfw_timestamp,
            sfw_version,
            tcg_iv_version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse() {
        const DATA_VALID: &[[&[u8; SIZE]; 2]] = &[
            [
                test_data::seagate_momentus5::ID_PAGE_0,
                test_data::seagate_momentus5::ID_PAGE_11,
            ],
            [
                test_data::seagate_barracudapro::ID_PAGE_0,
                test_data::seagate_barracudapro::ID_PAGE_11,
            ],
        ];
        const DATA_INVALID: &[&[u8; SIZE]] = &[&[0; _], &[0xFF; _]];

        for &[page_0_data, page_11_data] in DATA_VALID {
            let page_0 = Page0::try_from(page_0_data).unwrap();
            let page_11_version = page_0.directory[11].unwrap();
            Page11::parse(page_11_data, page_11_version).unwrap();
        }

        for &data in DATA_INVALID {
            assert!(Page0::try_from(data).is_err());
            assert!(Page11::parse(data, Page11::VERSION_1).is_err());
            assert!(Page11::parse(data, Page11::VERSION_2).is_err());
        }
    }
}
