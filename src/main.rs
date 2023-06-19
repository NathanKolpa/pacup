use std::fmt::{Debug, Formatter};
use std::fs::File;

use clap::{Parser, Subcommand};

use pacup::files::get_packagelist_file_path;

enum CliError {
    NoPackagelistFile,
    OpenPackagelistError(std::io::Error),
}

impl Debug for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::NoPackagelistFile => {
                writeln!(f, "No packagelist file found, tried the following paths (in order): ")?;
                writeln!(f, "\t$XDG_CONFIG_HOME/pacup/packagelist")?;
                writeln!(f, "\t$HOME/.packagelist")?;
                write!(f, "\t/etc/pacup/packagelist")?;
            }
            CliError::OpenPackagelistError(e) => {
                write!(f, "Cannot open packagelist file [{e}]")?;
            }
        }

        Ok(())
    }
}

#[derive(Parser)]
#[command(about = "Synchronise packages between packagelist and pacman")]
struct Cli {
    /// Print the difference between the host packages and the packagelist
    #[arg(short, long)]
    diff: bool,
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    let packagelist_path = get_packagelist_file_path().ok_or(CliError::NoPackagelistFile)?;

    let packagelist_file = File::options()
        .read(true)
        .open(packagelist_path)
        .map_err(|e| CliError::OpenPackagelistError(e))?;

    Ok(())
}
