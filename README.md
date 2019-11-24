
**This crate is longer being maintained on crates.io.**

**Because it is renamed to (lpfs)[https://github.com/pearzl/lpfs].**

You can find these code at proc_getter branch of lpfs.

-----

[![docs.rs](https://docs.rs/proc_getter/badge.svg)](https://docs.rs/proc_getter)
[![crates.io](https://img.shields.io/crates/v/proc_getter.svg)](https://crates.io/crates/proc_getter)
[![license](https://img.shields.io/github/license/pearzl/proc_getter)](./LICENSE)

The goal of this repo is to make retriving system informations reside in proc file system easier.

# Example

```
use proc_getter::cmdline::*;
use proc_getter::pid::cmdline::*;

fn main() {
    //  /proc/cmdline
    println!("{:?}", cmdline());

    //  /proc/1/cmdline
    println!("{:?}", cmdline_of(1));
}
```

## minial requirement

rust 2018 (rustc 1.31.0 +)

## supported

I try to make this support any distribution with 3.+ kernel version.

The proc file system is not the same among different version of kernel and distribution. 
Some code may not works on your system.

Therefore, almost every function return Result for the reason that specified file may not exist in your system.
It is possible that specidied file is exist but still return Err. I condider this case as a mistake and should be fix, PR and issue is welcomed.

Majority files listed [here](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-proc) shoule be supported.
Detail information is listed blow.

## process

#### top-level file

/proc/*

| file | status | file | status | file | status |
| --- | --- | --- | --- | --- | --- |
| buddyinfo| ✔ | cmdline | ✔ | consoles | ✔|
| cpuinfo | ✔ | crypto | ✔ | devices | ✔ |
| diskstates | ✔ | dma | ✔ | execdomains | ✔ |
| fb | ✔ | filesystems | ✔ | interrupts | ✔ |
| iomem | ✔ | ioports | ✔ | kcore | ✔ |
| kmsg | ❌ | loadavg| ✔ | locks | ✔ |
| mdstat | ✔ | meminfo | ✔ | misc | ✔ |
| modules | ✔ | mounts | ✔ | mtrr | ✔ |
| partitions | ✔ | slabinfo | ✔ | stat | ✔ |
| swaps | ✔ | uptime | ✔ | version | ✔ |
| self | ✔ | sysrq-trigger | ❌ |

#### process directroies

/proc/${pid}/*

| file | status | file | status | file | status |
| --- | --- | --- | --- | --- | --- | 
| cmdline| ✔ | cwd | ✔ | environ | ✔ |
| exe | ✔ | fd | ❌ | maps | ✔ |
| root | ✔ | stat | ✔ | statm | ✔ |
| status | ✔ |

#### other subdirectories

The table below lists the implemented files. 

| directory | file |
| --- | --- |
| acpi | wakeup |
| driver | rtc |
| net | arp, dev, dev_mcast, iptables_name, netstat |
|  | route |

*Note: Not fully tested.*

## LICENSE

[MIT](./LICENSE)
