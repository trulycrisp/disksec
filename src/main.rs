//! CLI entry point.

mod cpu;
mod drive;
mod flash_id;
mod os;
mod output;
mod protocol;
mod test_data;

use std::{
    io,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Parser;

/// Main error, transparently wraps other error types.
#[derive(Debug)]
pub enum Error {
    /// Drive-level error.
    Drive(drive::Error),
    /// Error writing output.
    Io(io::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Drive(x) => x.source(),
            Self::Io(x) => x.source(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Drive(x) => x.fmt(f),
            Self::Io(x) => x.fmt(f),
        }
    }
}

impl From<drive::Error> for Error {
    fn from(value: drive::Error) -> Self {
        Self::Drive(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

/// CLI subcommand.
#[derive(clap::Subcommand)]
enum Action {
    /// List available drives.
    List,
    /// Run drive checks.
    Run {
        /// Drive to check, checks all if omitted.
        path: Option<PathBuf>,
    },
}

/// Storage drive firmware security assessment.
#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    /// Subcommand.
    #[command(subcommand)]
    action: Action,
    /// Log verbosity level.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
    /// Optional file path to write output to, in addition to stdout.
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,
}

/// Initialize logging.
fn init_log(level: u8) {
    let level_filter = match level {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .parse_default_env()
        .filter_level(level_filter)
        .init();
}

/// List available drives.
fn list(output: &mut dyn io::Write) -> Result<(), Error> {
    let mut drives = drive::Drive::open_all()?;

    output::list(&mut drives, output)?;

    Ok(())
}

/// Run checks on a drive or all available drives.
fn run(output: &mut dyn io::Write, path: Option<&Path>) -> Result<(), Error> {
    if let Some(path) = path {
        let drive = drive::Drive::open(path)?;
        let mut drives = std::iter::once(drive::OpenAllItem::Ok(drive));
        output::run(&mut drives, output)?;
    } else {
        let mut drives = drive::Drive::open_all()?;
        output::run(&mut drives, output)?;
    }

    Ok(())
}

/// Handle CLI argument executing appropriate functionality.
fn handle_arguments(arguments: Arguments) -> Result<(), Error> {
    init_log(arguments.verbose);

    let mut output = Vec::<u8>::new();
    output::header(&mut output)?;

    match arguments.action {
        Action::List => list(&mut output)?,
        Action::Run { path } => run(&mut output, path.as_deref())?,
    }

    output::write_output(output.as_slice(), arguments.output.as_deref())?;

    Ok(())
}

/// CLI entry point.
fn main() -> ExitCode {
    let arguments = Arguments::parse();

    if let Err(error) = handle_arguments(arguments) {
        eprintln!("Error: {}", output::format_error(&error));
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
