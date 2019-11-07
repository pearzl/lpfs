#![cfg(target_os = "linux")]
#![allow(dead_code)]

//! This crate interoperate files under /proc/ in linux.
//!
//! ```
//! use linux_proc::proc::cpuinfo::*;
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

pub mod proc;
