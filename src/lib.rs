#![cfg(any(target_os = "linux", target_os = "android"))]

/// define some macro_rule used in this crate.
#[macro_use]
mod macros;

/// all the funcitons return this error in the crate.
#[derive(Debug)]
pub enum ProcErr {
    /// Failed to read the corresponding file.
    IO(std::io::Error),
}

#[cfg(any(feature = "all", feature = "proc"))]
pub mod proc;
