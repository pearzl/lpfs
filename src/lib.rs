#![cfg(target_os = "linux")]

//! This crate interoperate files under /proc/ in linux.
//!
//! ```
//! use linux_proc::*;
//!
//! fn main() {
//!     println!("{:?}", cpuinfo().unwrap());
//! }
//! ```
//!
//! ## layout of this crate
//!
//! It's very like the /proc/ directory layout.
//!
//! Top-level files within proc file system has a function to interoperate with
//! which has the same name of the files.
//! For example, read information from /proc/cpuinfo should use `crate::proc::cpuinfo()`.
//!
//! Those files reside in directories within /proc/ has a same name mod.
//! For example, /proc/driver/rtc and `crate::driver::rtc()`.
//!
//! Everything under proc mod is re-export, so crate::proc is not necessary.
//!
//! ## Returning value
//!
//! The returning value has different type depends on the files.
//! Some function simply return the content of the file, including ending `\n`.
//! Some function parse the content and return the light wrapping.
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
//! Here is a list from [red hat], and every file in this list is supported now.
//!
//![red hat]: https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-proc

#[derive(Debug)]
/// All errors in this crate.
///
/// In most cases, all function should success,
/// for everythin reside in `/proc/` is not a real file.
/// Returning error is only exist in theory
/// if the file exist on your computer as we using std::io inside.
pub enum Error {
    /// contains a std::io::Error, which should be `NotFound`.
    IO(std::io::Error),
    /// this error should appear.
    /// Otherwise it's a situation where unexpected input appears.
    /// That is a bug.default_read!
    BadFormat,
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
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

/// return the origin content
macro_rules! default_read {
    ($fn_name: ident, $path: expr) => {
        /// Read the whole file content and return it.
        /// Ending line break is included.
        pub fn $fn_name() -> Result<String> {
            Ok(std::fs::read_to_string($path)?)
        }
    };
}

/// content is divided into some block, each block is consist of many key-value pairs.
macro_rules! default_pairs {
    ($fn_name: ident, $path: expr, $return_value: expr) => {
        default_pairs! {$fn_name, $path, $return_value, "\n\n", ':'}
    };
    ($fn_name: ident, $path: expr, $return_value: expr, $sep_block: expr, $sep_pair: expr) => {
        #[doc="Each "]
        #[doc=$return_value]
        #[doc=" is represented by a HashMap."]
        pub fn $fn_name() -> Result<Vec<std::collections::HashMap<String, String>>> {
            let content = std::fs::read_to_string($path)?;
            let mut ret = vec![];

            for processor in content.split($sep_block) {
                let mut map = std::collections::HashMap::new();
                for line in processor.lines() {
                    let mut kv = line.split($sep_pair);
                    let key = kv.next().ok_or(Error::BadFormat)?;
                    let value = kv.next().ok_or(Error::BadFormat)?;
                    map.insert(key.trim().to_string(), value.trim().to_string());
                }
                if !map.is_empty() {
                    ret.push(map);
                }
            }

            Ok(ret)
        }
    };
}

/// generate a simple unit test which print the returning value to std_out.
#[cfg(test)]
macro_rules! output_unit_test {
    ($fn_name: ident) => {
        #[test]
        fn $fn_name() {
            println!("{:#?}", super::$fn_name().unwrap());
        }
    };
}

pub mod proc;

#[doc(inline)]
pub use proc::*;
