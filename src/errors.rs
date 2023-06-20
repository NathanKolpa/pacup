use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::process::ExitStatus;

pub enum CliError {
    NoPackagelistFile,
    OpenPackagelistError {
        path: PathBuf,
        error: std::io::Error,
    },
    FetchPackagesError(PacmanError),
    ReadPackagelistError(PackageListError),
    InstallError(PacmanError),
}

impl Debug for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::NoPackagelistFile => {
                writeln!(
                    f,
                    "No packagelist file found, tried the following paths (in order): "
                )?;
                writeln!(f, "\t$XDG_CONFIG_HOME/pacup/packagelist")?;
                writeln!(f, "\t$HOME/.packagelist")?;
                write!(f, "\t/etc/pacup/packagelist")?;
            }
            CliError::OpenPackagelistError { error, path } => {
                write!(
                    f,
                    "Cannot open packagelist file {} ({error})",
                    path.display()
                )?;
            }
            CliError::FetchPackagesError(e) => {
                write!(f, "Failed to fetch pacman packages: {e:?}")?;
            }
            CliError::ReadPackagelistError(e) => {
                write!(f, "Error while reading packagelist: {e:?}")?;
            }
            CliError::InstallError(e) => {
                write!(f, "Failed to install packages: {e:?}")?;
            }
        }

        Ok(())
    }
}

pub enum PacmanError {
    CannotRunCommand(std::io::Error),
    CannotWaitForChild(std::io::Error),
    NonZeroExit(ExitStatus),
}

impl Debug for PacmanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacmanError::CannotRunCommand(e) => write!(f, "Cannot run the specified command: {e}"),
            PacmanError::CannotWaitForChild(e) => {
                write!(f, "Error while waiting for child process: {e}")
            }
            PacmanError::NonZeroExit(e) => write!(f, "Non zero exit status ({e})"),
        }
    }
}

pub enum PackageLineParseError {
    ExpectedPackageType,
    UnknownPackageType(String),
    ExpectedPackageName,
    UnexpectedString(String),
}

impl Debug for PackageLineParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageLineParseError::ExpectedPackageType => write!(f, "Expected package type"),
            PackageLineParseError::UnknownPackageType(value) => write!(
                f,
                "Unknown package type, expected '+' or '*' got \"{value}\""
            ),
            PackageLineParseError::ExpectedPackageName => write!(f, "Expected package name"),
            PackageLineParseError::UnexpectedString(value) => {
                write!(f, "Unexpected string \"{value}\"")
            }
        }
    }
}

pub enum PackageListError {
    ReadFileError(std::io::Error),
    LineParseError(usize, PackageLineParseError),
}

impl Debug for PackageListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageListError::ReadFileError(e) => write!(f, "Failed to read file: {e}"),
            PackageListError::LineParseError(line, e) => {
                write!(f, "Parse error at line {line} {e:?}")
            }
        }
    }
}
