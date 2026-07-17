//! Generic drive interface.

pub mod phison;
pub mod seagate;
pub mod wd;

use std::path::{Path, PathBuf};

use log::debug;

use crate::{
    os,
    protocol::{ata, scsi},
};

/// Drive error.
#[derive(Debug)]
pub enum Error {
    /// OS error.
    Os(os::Error),
    /// SCSI error.
    Scsi(scsi::Error),
    /// ATA error.
    Ata(ata::Error),
    /// Not a valid/supported drive.
    InvalidDrive,
    /// Vendor-specific error.
    Vendor(Box<dyn VendorError>),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Os(x) => Some(x),
            Self::Scsi(x) => Some(x),
            Self::Ata(x) => Some(x),
            Self::InvalidDrive => None,
            Self::Vendor(x) => Some(x.as_ref()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Os(_) => write!(f, "OS error"),
            Self::Scsi(_) => write!(f, "SCSI error"),
            Self::Ata(_) => write!(f, "ATA error"),
            Self::InvalidDrive => write!(f, "invalid drive"),
            Self::Vendor(x) => write!(f, "{} error", x.name()),
        }
    }
}

impl From<os::Error> for Error {
    fn from(value: os::Error) -> Self {
        Self::Os(value)
    }
}

impl From<scsi::Error> for Error {
    fn from(value: scsi::Error) -> Self {
        match value {
            scsi::Error::Os(x) => x.into(),
            x => Self::Scsi(x),
        }
    }
}

impl From<ata::Error> for Error {
    fn from(value: ata::Error) -> Self {
        match value {
            ata::Error::Scsi(scsi::Error::Os(x)) => x.into(),
            x => Self::Ata(x),
        }
    }
}

impl From<Box<dyn VendorError>> for Error {
    fn from(value: Box<dyn VendorError>) -> Self {
        Self::Vendor(value)
    }
}

/// Vendor type check error.
pub trait VendorError: std::error::Error {
    /// Get display name.
    fn name(&self) -> &str;
}

/// Item of the iterator returned by `Drive::open_all`.
pub type OpenAllItem = Result<Drive, (PathBuf, Error)>;

/// Drive interface.
#[derive(Debug)]
pub enum Drive {
    /// SCSI drive.
    Scsi(scsi::Drive),
    /// ATA drive.
    Ata(ata::Drive),
}

impl Drive {
    /// Open drive.
    pub fn open(path: &Path) -> Result<Self, Error> {
        let opened = |x| {
            debug!("[{x}] Opened");
            Ok(x)
        };

        match scsi::Drive::open(path) {
            Ok(x) => return opened(Self::try_from(x)?),
            Err(scsi::Error::UnsupportedDrive) => {},
            Err(e) => return Err(e.into()),
        }

        Err(Error::InvalidDrive)
    }

    /// Enumerate and open all available drives.
    pub fn open_all() -> Result<impl Iterator<Item = OpenAllItem>, Error> {
        let paths = os::list_drives()?;

        let drives = paths
            .into_iter()
            .filter_map(|path| match Self::open(&path) {
                Ok(x) => Some(Ok(x)),
                Err(Error::InvalidDrive) => None,
                Err(x) => Some(Err((path, x))),
            });

        Ok(drives)
    }

    /// Run checks on drive.
    pub fn check(&self) -> Result<Box<[VendorResult]>, Error> {
        let mut vendor_results = Vec::new();

        for vendor_type in VENDOR_TYPES {
            let vendor_name = vendor_type.name();
            debug!("[{self}] Starting vendor check: {vendor_name}");

            let vendor_type_result = vendor_type.check(self)?;
            debug!("[{self}] Vendor type result: {vendor_name} ({vendor_type_result:?})");

            if let Some(check_results) = vendor_type_result {
                vendor_results.push(VendorResult {
                    name: vendor_name.into(),
                    results: check_results,
                });
            }
        }

        Ok(vendor_results.into())
    }

    /// Display drive type and information
    pub fn display_info(&self) -> Result<String, Error> {
        let info = match self {
            Self::Scsi(x) => x.inquiry()?.to_string(),
            Self::Ata(x) => x.identify_device()?.to_string(),
        };

        Ok(format!("{self} {info}"))
    }
}

impl TryFrom<scsi::Drive> for Drive {
    type Error = Error;

    fn try_from(scsi_drive: scsi::Drive) -> Result<Self, Self::Error> {
        // Try convert to ATA drive to check if ATA (SAT)
        match ata::Drive::try_from(scsi_drive) {
            Ok(ata_drive) => Ok(ata_drive.into()),
            Err(ata::Error::UnsupportedDrive(scsi_drive)) => Ok(Self::Scsi(scsi_drive)),
            Err(x) => Err(x.into()),
        }
    }
}

impl From<ata::Drive> for Drive {
    fn from(value: ata::Drive) -> Self {
        Self::Ata(value)
    }
}

impl std::fmt::Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scsi(x) => x.fmt(f),
            Self::Ata(x) => x.fmt(f),
        }
    }
}

/// Result of single check for vendor type.
#[derive(Debug)]
pub struct CheckResult {
    /// Check display name.
    pub name: String,
    /// Check display result.
    pub result: Result<String, Error>,
}

impl CheckResult {
    /// Construct check result.
    pub fn new(name: String, result: Result<String, Error>) -> Self {
        Self { name, result }
    }
}

/// Set of check results for vendor type.
#[derive(Debug)]
pub struct VendorResult {
    /// Vendor type display name.
    pub name: String,
    /// Check results.
    pub results: Box<[CheckResult]>,
}

/// Vendor-specific drive type.
pub trait VendorType {
    /// Get display name.
    fn name(&self) -> &str;
    /// Run checks on drive.
    fn check(&self, drive: &Drive) -> Result<Option<Box<[CheckResult]>>, Error>;
}

/// All vendor types.
const VENDOR_TYPES: &[&dyn VendorType] = &[
    &phison::ata::s5::VendorType,
    &phison::ata::s8::VendorType,
    &phison::ata::s9::VendorType,
    &phison::ata::s10::VendorType,
    &phison::ata::s11::VendorType,
    &phison::ata::s12::VendorType,
    &seagate::ata::f3::VendorType,
    &wd::ata::marvell::VendorType,
];
