use std::process::{Command, ExitStatus};

pub enum Swap {
    On,
    Off,
}

impl Swap {
    #[must_use]
    pub const fn switch(&self) -> &'static str {
        match self {
            Self::On => "swapoff",
            Self::Off => "swapon",
        }
    }

    /// # Errors
    /// Returns an error if the `sudo` command cannot be executed
    pub fn execute(&self) -> Result<ExitStatus, std::io::Error> {
        Command::new("sudo")
            .arg(self.switch())
            .arg("-a")
            .stdout(std::process::Stdio::null())
            .status()
    }
}
