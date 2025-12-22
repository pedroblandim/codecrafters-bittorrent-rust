use std::str::FromStr;

pub mod decode;

pub enum Commands {
    Decode,
}

impl FromStr for Commands {
    type Err = ();
    fn from_str(command: &str) -> Result<Self, Self::Err> {
        if command == "decode" {
            Ok(Commands::Decode)
        } else {
            Err(())
        }
    }
}
