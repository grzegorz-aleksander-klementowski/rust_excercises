use rpassword::read_password;
use std::process::{Command, ExitStatus};

pub enum Swap {
    On,
    Off,
}

impl Swap {
    pub fn switch(&self) -> &'static str {
        match self {
            Swap::On => "swapoff",
            Swap::Off => "swapon",
        }
    }

    pub fn execute(&self) -> Result<ExitStatus, std::io::Error> {
        Command::new("sudo")
            .arg(self.switch())
            .arg("-a")
            .stdout(std::process::Stdio::null())
            .status()
    }
}
