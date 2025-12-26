use std::str::FromStr;

pub enum Commands {
    Decode,
    Info,
    Peers,
}

impl FromStr for Commands {
    type Err = String;
    fn from_str(command: &str) -> Result<Self, Self::Err> {
        match command {
            "decode" => Ok(Commands::Decode),
            "info" => Ok(Commands::Info),
            "peers" => Ok(Commands::Peers),
            other => Err(format!("Invalid command {other}")),
        }
    }
}
