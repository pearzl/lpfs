#![cfg(any(target_os = "linux", target_os = "android"))]

/// define some macro_rule used in this crate.
#[macro_use]
mod macros;

pub mod proc;
pub mod pid;
pub mod _self;

/// all the funcitons return this error in the crate.
#[derive(Debug)]
pub enum ProcErr {
    /// Failed to read the corresponding file.
    IO(std::io::Error),
    Parse(Box<dyn std::error::Error>),
    BadFormat(String),
}

impl From<std::io::Error> for ProcErr {
    fn from(x: std::io::Error) -> Self {
        ProcErr::IO(x)
    }
}

impl From<std::num::ParseIntError> for ProcErr {
    fn from(x: std::num::ParseIntError) -> Self {
        ProcErr::Parse(Box::new(x))
    }
}

impl From<std::num::ParseFloatError> for ProcErr {
    fn from(x: std::num::ParseFloatError) -> Self {
        ProcErr::Parse(Box::new(x))
    }
}
