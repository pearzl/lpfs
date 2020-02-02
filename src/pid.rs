//! Contains the process sub-directory files within the proc file system.
//!
//! Each file has its own submodule with the same name.
//!
//!

pub type Pid = u32;
pub type Tid = u32;

pub mod task;
pub mod stat;
pub mod comm;
pub mod fd;
