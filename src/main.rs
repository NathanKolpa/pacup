use std::fs::File;

use clap::Parser;
use pacup::errors::CliError;

use pacup::files::get_packagelist_file_path;
use pacup::paclist::{PackageLineKind, PackageListReader};
use pacup::pacman::Pacman;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Synchronise packages between packagelist and pacman")]
struct Cli {
    /// Print out the packages that would be installed without actually installing
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    let packagelist_path = get_packagelist_file_path().ok_or(CliError::NoPackagelistFile)?;

    let packagelist_file = File::options()
        .read(true)
        .open(&packagelist_path)
        .map_err(|e| CliError::OpenPackagelistError {
            error: e,
            path: packagelist_path,
        })?;

    let mut pacman = Pacman::new();
    pacman.fetch_user_status();

    let mut reader = PackageListReader::new(packagelist_file);

    let mut current_packages_buffer = String::new();
    let current_packages = pacman
        .fetch_all_installed_packages(&mut current_packages_buffer)
        .map_err(|e| CliError::FetchPackagesError(e))?;

    let mut transaction = pacman.begin_transaction();

    while let Some(line) = reader.next_line_not_in_set(&current_packages) {
        let line = line.map_err(|e| CliError::ReadPackagelistError(e))?;
        transaction.add_package(line.name().to_string());

        match line.kind() {
            PackageLineKind::Aur => transaction.make_aur_transaction(),
            _ => {}
        }
    }

    if transaction.is_empty() {
        eprintln!("All packages installed, there is nothing to do!");
        return Ok(());
    }
    eprintln!(
        "Installing {} missing package(s)",
        transaction.missing_package_count()
    );
    eprintln!("{transaction:?}");

    if cli.dry_run {
        for package in transaction.iter_packages() {
            println!("{package}");
        }

        Ok(())
    } else {
        Err(CliError::InstallError(transaction.commit()))
    }
}
