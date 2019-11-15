// #![cfg(target_os = "linux")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![allow(dead_code)]


//! ## how to use
//!
//! The layout of this crate is very like the /proc/ directory layout.
//! 
//! Each file correspond to a module which contains everything to interoperate with the file.
//! Majority modules has a function which has the same name as the file.
//! 
//! However, function reside in process directories is a little different. 
//! They receive an u32 argument to specify the target process, and the function name has a "_of" suffix.
//!
//! ```
//! use crate::cmdline::*;
//! use crate::pid::cmdline::*;
//! 
//! fn main() {
//!     //  /proc/cmdline
//!     println!("{:?}", cmdline());
//!     
//!     //  /proc/1/cmdline
//!     println!("{:?}", cmdline_of(1));
//! }
//! ```
//!
//! Everything under proc mod is re-export, so `crate::proc::cmdline` equals to `crate::cmdline`.
//!
//! ## Returning value
//!
//! The returning value has different type depends on the files.
//! Some function simply return the content of the file, including ending `\n`.
//! Some function parse the content and return the wrapping type.
//!
//! There are tow possbile cases where return an Err:
//!
//! 1. there is a bug because we parse the file content incorrectlly.
//! 2. the file corresponding to the function is not exist in your computer.
//!
//! Otherwise all the function should success in theory,
//! since files under proc file system are not real file.
//!
//! ## which files is supported
//!
//! It's very depends on th system.
//! Here is a list from [red hat], every file in this list should be supported.
//!
//![red hat]: https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-proc

/// all kind of error may occurs in the crate.
/// 
/// IO often occurs when the file is not exists in your system, it wrap an std::io::Error::NotFound.
/// others should be consider as an error and should be fix.
#[derive(Debug)]
pub enum Error {
    /// contains a std::io::Error, which should be `NotFound`.
    IO(std::io::Error),
    /// this error should appear.
    /// Otherwise it's a situation where unexpected input appears.
    /// That is a bug.default_read!
    BadFormat,
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    ParseAddr(std::net::AddrParseError),
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::ParseAddr(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

macro_rules! default_read {
    ($fn_name: ident, $path: expr) => {
        /// Read the whole file content and return it.
        /// Ending line break is included.
        pub fn $fn_name() -> $crate::Result<String> {
            Ok(std::fs::read_to_string($path)?)
        }
    };
}

macro_rules! default_list {
    ($fn_name: ident, $path: expr, $return_type: ty, $parse_code: ident, $sep: expr, $skip: expr) => {
        pub fn $fn_name() -> $crate::Result<Vec<$return_type>> {
            let content = std::fs::read_to_string($path)?;
            let mut ret = vec![];
            let mut block_iter = content.trim().split($sep);
            for _ in 0..$skip {
                let _ = block_iter.next();
            }
            for block in block_iter {
                ret.push($parse_code(block)?);
            }
            Ok(ret)
        }
    };
    ($fn_name: ident, $path: expr, $return_type: ty, $parse_code: ident, $sep: expr) => {
        default_list! {$fn_name, $path, $return_type, $parse_code, $sep, 0}
    };
    ($fn_name: ident, $path: expr, $return_type: ty, $parse_code: ident) => {
        default_list! {$fn_name, $path, $return_type, $parse_code, '\n', 0}
    };
}

macro_rules! getter_gen {
    (
        $(
            $filed: ident : $type: ty
        ), *
    ) => {
        $(
            #[inline(always)]
            pub fn $filed(&self) -> &$type {
                &self.$filed
            }
        ) *
    };
}

#[doc(inline)]
pub mod proc;
pub use proc::*;
