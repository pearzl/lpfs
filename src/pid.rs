//! Contains the process sub-directory files within the proc file system.
//!
//! Each file has its own submodule with the same name.
//!
//!

pub type Pid = u32;
pub type Tid = u32;

pub mod cmdline;
pub mod comm;
pub mod cwd;
pub mod fd;
pub mod stat;
pub mod task;
