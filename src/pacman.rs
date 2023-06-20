use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, ExitStatus, Stdio};

pub enum PacmanError {
    BinaryNotFound(String),
    CannotRunCommand(std::io::Error),
    CannotWaitForChild(std::io::Error),
    NonZeroExit(ExitStatus),
}

pub struct Pacman {
    binary: &'static str,
    aur_binary: &'static str,
    default_to_aur: bool,
}

impl Pacman {
    pub fn new() -> Self {
        Self {
            aur_binary: "/usr/bin/trizen",
            binary: "/usr/bin/pacman",
            default_to_aur: false,
        }
    }

    pub fn all_installed_packages<'a>(&self, str_buffer: &'a mut OsString) -> Result<HashSet<&'a OsStr>, PacmanError> {
        let child = self.run_command(true, self.binary, ["-Qnq"])?;

        let output = child
            .wait_with_output()
            .map_err(|e| PacmanError::CannotWaitForChild(e))?;

        if !output.status.success() {
            return Err(PacmanError::NonZeroExit(output.status));
        }

        *str_buffer = OsString::from(output.stdout);

        Ok(str_buffer.lines().collect())
    }

    fn run_command<I: IntoIterator<Item=S>, S: AsRef<OsStr>>(
        &self,
        collect_stdout: bool,
        name: &str,
        args: I,
    ) -> Result<Child, PacmanError> {
        Command::new(name)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(if collect_stdout {
                Stdio::piped()
            } else {
                Stdio::inherit()
            })
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| PacmanError::CannotRunCommand(e))
    }
}
