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

macro_rules! default_read {
    ($fn_name: ident, $path: expr) => {
        /// Read the whole file content and return it.
        /// Ending line break is included.
        pub fn $fn_name() -> $crate::proc::Result<String> {
            Ok(std::fs::read_to_string($path)?)
        }
    };
}

macro_rules! default_list {
    ($fn_name: ident, $path: expr, $return_type: ty, $parse_code: ident, $sep: expr, $skip: expr) => {
        pub fn $fn_name() -> $crate::proc::Result<Vec<$return_type>> {
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
            pub fn $filed(&self) -> &$type {
                &self.$filed
            }
        ) *
    };
}

mod buddyinfo;
mod cmdline {
    default_read! {cmdline, "/proc/cmdline"}
}
mod cpuinfo;
mod crypto;
mod dma;
mod execdomains {
    default_read! {execdomains, "/proc/execdomains"}
}
mod fb;
mod filesystems;
mod interrupts;
mod iomem;
mod kcore;
mod loadavg;
mod kmsg {
    /// Unimplemented now.
    pub fn kmsg() -> super::Result<String> {
        unimplemented!()
    }
}
mod locks;
mod mdstat {
    default_read! {mdstat, "/proc/mdstat"}
}
mod meminfo;
mod misc;
mod consoles {
    default_read! {consoles, "/proc/consoles"}
}
pub mod modules;
pub mod mounts;
pub mod mtrr {
    default_read! {mtrr, "/proc/consoles"}
}
pub mod acpi;
pub mod driver;
pub mod partitions;
