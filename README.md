[![docs.rs](https://docs.rs/lpfs/badge.svg)](https://docs.rs/lpfs)
[![crates.io](https://img.shields.io/crates/v/lpfs.svg)](https://crates.io/crates/lpfs)
[![license](https://img.shields.io/github/license/pearzl/lpfs)](./LICENSE)


The goal of this repo is to make retriving system informations reside in proc file system easier.

# Example

```
use lpfs::proc::*;
use lpfs::pid::*;

fn main() {
    //  /proc/stat
    println!("{:?}", stat().unwrap());

    //  /proc/1/stat
    println!("{:?}", stat_of(1).unwrap());
}
```

## Interface Design

[eminence/procfs](https://github.com/eminence/procfs) is a similar crate with different interface design.

#### Top-Level Files

Each file is retrived by a function reside in `lpfs::proc` with same name. 
For example, `lpfs::proc::cmdline` retrive from `/proc/cmdline`.

#### Process Directories

It's similar with top-level files, here is an example with `stat`.

| function | file |
| --- | --- |
| stat_of(pid) | /proc/[pid]/stat |
| stat_self() | /proc/self/stat |
| stat_of_task(pid, tid) | /proc/[pid]/task/[tid]/stat |
| stat_self_task(tid) | /proc/self/task/[tid]/stat |

These functions reside in `lpfs::pid`

## Minial Requirement

rust 2018 (rustc 1.31.0 +)

## Supported

all linux distribution with 3.+ kernel version.

## LICENSE

[MIT](./LICENSE)
