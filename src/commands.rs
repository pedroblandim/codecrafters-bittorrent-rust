use std::str::FromStr;

pub mod decode;

pub enum Commands {
    Decode,
    Info,
}

impl FromStr for Commands {
    type Err = String;
    fn from_str(command: &str) -> Result<Self, Self::Err> {
        match command {
            "decode" => Ok(Commands::Decode),
            "info" => Ok(Commands::Info),
            other => Err(format!("Invalid command {other}")),
        }
    }
}
