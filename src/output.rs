//! High-level user-facing output functionality.
use std::{io, path::Path};

use crate::drive;

/// Display error.
pub fn format_error(mut error: &dyn std::error::Error) -> String {
    use std::fmt::Write as _;
    let mut string = error.to_string();
    while let Some(source) = error.source() {
        write!(string, ": {source}").unwrap();
        error = source;
    }
    string
}

/// Write output header.
pub fn header(output: &mut dyn io::Write) -> io::Result<()> {
    writeln!(output, "DiskSec v{}\n", env!("CARGO_PKG_VERSION"))
}

/// Output single item from `drive::Drive::open_all`, dispatch to `function` on
/// success, or output the error.
fn output_drive(
    drive_result: drive::OpenAllItem,
    output: &mut dyn io::Write,
    function: impl FnOnce(&drive::Drive, &mut dyn io::Write) -> io::Result<()>,
) -> io::Result<()> {
    match drive_result {
        Ok(drive) => function(&drive, output),
        Err((p, e)) => writeln!(output, "{}: {}", p.display(), format_error(&e)),
    }
}

/// Output information for drive.
fn list_drive(drive: &drive::Drive, output: &mut dyn io::Write) -> io::Result<()> {
    let line = drive
        .display_info()
        .unwrap_or_else(|e| format!("{drive}: {}", format_error(&e)));

    writeln!(output, "{line}")
}

/// Enumerate drives and output their information.
pub fn list(
    drives: &mut dyn Iterator<Item = drive::OpenAllItem>,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    for drive_result in drives {
        output_drive(drive_result, output, list_drive)?;
    }

    Ok(())
}

/// Run checks on a drive and output results.
fn run_drive(drive: &drive::Drive, output: &mut dyn io::Write) -> io::Result<()> {
    let info = match drive.display_info() {
        Ok(info) => info,
        Err(e) => return writeln!(output, "{drive}: {}", format_error(&e)),
    };

    writeln!(output, "{info}:")?;

    let vendor_results = match drive.check() {
        Ok(results) => results,
        Err(e) => return writeln!(output, "\tError: {}", format_error(&e)),
    };

    if vendor_results.is_empty() {
        writeln!(output, "\tN/A")?;
    }

    for vendor_result in vendor_results {
        writeln!(output, "\t{}:", vendor_result.name)?;

        if vendor_result.results.is_empty() {
            writeln!(output, "\t\tN/A")?;
        }

        for check_result in vendor_result.results {
            writeln!(output, "\t\t{}: {}", check_result.name, check_result.result)?;
        }
    }

    Ok(())
}

/// Run checks on drives and output results.
pub fn run(
    drives: &mut dyn Iterator<Item = drive::OpenAllItem>,
    output: &mut dyn io::Write,
) -> io::Result<()> {
    let mut first = true;

    for drive_result in drives {
        if !std::mem::take(&mut first) {
            // Blank line separating drives
            writeln!(output)?;
        }

        output_drive(drive_result, output, run_drive)?;
    }

    Ok(())
}

/// Write output, to standard output and optionally a file.
pub fn write_output(output: &[u8], file_path: Option<&Path>) -> io::Result<()> {
    use std::io::Write as _;

    io::stdout().write_all(output)?;

    if let Some(path) = file_path {
        std::fs::write(path, output)?;
    }

    Ok(())
}
