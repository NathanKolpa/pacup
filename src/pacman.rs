use crate::errors::PacmanError;
use core::fmt::Formatter;
use std::collections::HashSet;
use std::fmt::Debug;
use std::process::{Command, Stdio};

pub struct Pacman {
    binary: &'static str,
    aur_binary: &'static str,
    sudo_binary: &'static str,
    default_to_aur: bool,
    is_root: bool,
}

impl Pacman {
    pub fn new() -> Self {
        Self {
            aur_binary: "/usr/bin/trizen",
            binary: "/usr/bin/pacman",
            sudo_binary: "/usr/bin/sudo",
            default_to_aur: false,
            is_root: false,
        }
    }

    pub fn fetch_user_status(&mut self) {
        let uid = unsafe { libc::getuid() };
        self.is_root = uid == 0;
    }

    pub fn fetch_all_installed_packages<'a>(
        &self,
        str_buffer: &'a mut String,
    ) -> Result<HashSet<&'a str>, PacmanError> {
        let child = Command::new(self.binary)
            .arg("-Qnq")
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| PacmanError::CannotRunCommand(e))?;

        let output = child
            .wait_with_output()
            .map_err(|e| PacmanError::CannotWaitForChild(e))?;

        if !output.status.success() {
            return Err(PacmanError::NonZeroExit(output.status));
        }

        // Safety: We just want to compare two strings, it should not matter if it is valid UTF-8.
        *str_buffer = unsafe { String::from_utf8_unchecked(output.stdout) };

        Ok(str_buffer.lines().collect())
    }

    pub fn begin_transaction(&self) -> PackageTransaction<'_> {
        PackageTransaction {
            pacman: self,
            packages_to_add: Vec::new(),
            should_use_aur: false,
        }
    }
}

pub struct PackageTransaction<'a> {
    pacman: &'a Pacman,
    packages_to_add: Vec<String>,
    should_use_aur: bool,
}

impl PackageTransaction<'_> {
    pub fn add_package(&mut self, package: String) {
        self.packages_to_add.push(package);
    }

    pub fn make_aur_transaction(&mut self) {
        self.should_use_aur = true;
    }

    fn should_use_aur(&self) -> bool {
        self.should_use_aur || self.pacman.default_to_aur
    }

    fn install_opts(&self) -> &str {
        "-Sy"
    }

    fn binary_name(&self) -> &str {
        if self.should_use_aur() {
            self.pacman.aur_binary
        } else {
            self.pacman.binary
        }
    }

    pub fn is_empty(&self) -> bool {
        self.packages_to_add.is_empty()
    }

    pub fn missing_package_count(&self) -> usize {
        self.packages_to_add.len()
    }

    fn should_sudo(&self) -> bool {
        !self.pacman.is_root && !self.should_use_aur
    }

    pub fn commit(self) -> PacmanError {
        use std::os::unix::process::CommandExt;

        let mut command = if self.should_sudo() {
            let mut sudo_cmd = Command::new(self.pacman.sudo_binary);
            sudo_cmd.arg(self.binary_name());
            sudo_cmd
        } else {
            Command::new(self.binary_name())
        };

        command.arg(self.install_opts()).args(self.packages_to_add);

        PacmanError::CannotRunCommand(command.exec())
    }

    pub fn iter_packages(&self) -> impl Iterator<Item = &str> {
        self.packages_to_add.iter().map(|s| s.as_str())
    }
}

impl Debug for PackageTransaction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.should_sudo() {
            write!(f, "{} ", self.pacman.sudo_binary)?;
        }

        write!(f, "{} ", self.binary_name())?;
        write!(f, "{} ", self.install_opts())?;

        for pkg in self.packages_to_add.iter() {
            write!(f, "{pkg} ")?;
        }

        Ok(())
    }
}
