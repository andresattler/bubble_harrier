use std::error::Error;
use std::fmt::*;
use std::result::Result as Res;

pub type SimResult = Res<(), SimError>;

#[derive(Debug, Clone, Copy)]
pub enum SimError {
    GeneralError,
}

impl Display for SimError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl Error for SimError {
    fn description(&self) -> &str {
        match self {
            GeneralError => "A general simulation error occured.",
        }
    }
}
