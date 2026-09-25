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

/// Display byte slice as hex.
pub fn format_bytes_hex(data: &[u8]) -> String {
    data.iter()
        .map(|x| format!("{x:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format byte size helper.
fn format_byte_size(mut size: u64, step: u64, units: &[&str]) -> String {
    let mut remainder = 0;
    let mut unit = 0;

    while size >= step && unit < units.len() - 1 {
        remainder = size % step;
        size /= step;
        unit += 1;
    }

    // The remainder is a fraction of a step, scale it to hundredths of the unit
    let fraction = remainder * 100 / step;

    format!("{size}.{fraction:02} {}", units[unit])
}

/// Display size in byte units, decimal prefixes.
pub fn format_byte_size_decimal(size: u64) -> String {
    const STEP: u64 = 1000;
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    format_byte_size(size, STEP, UNITS)
}

/// Display size in byte units, binary prefixes.
pub fn format_byte_size_binary(size: u64) -> String {
    const STEP: u64 = 1024;
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

    format_byte_size(size, STEP, UNITS)
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
            writeln!(
                output,
                "\t\t{}: {}",
                check_result.name,
                check_result
                    .result
                    .unwrap_or_else(|x| format!("Error ({})", format_error(&x)))
            )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_size_format() {
        assert_eq!(format_byte_size_decimal(0), "0.00 B");
        assert_eq!(format_byte_size_decimal(512), "512.00 B");
        assert_eq!(format_byte_size_decimal(500_107_862_016), "500.10 GB");
        // A fraction below a tenth of the unit keeps its leading zero
        assert_eq!(format_byte_size_decimal(1_073_741_824), "1.07 GB");
        // The largest unit is not exceeded
        assert_eq!(format_byte_size_decimal(u64::MAX), "18.44 EB");

        assert_eq!(format_byte_size_binary(1_073_741_824), "1.00 GiB");
        assert_eq!(format_byte_size_binary(1_500_000_000), "1.39 GiB");
    }
}
