// This file lists the _execution domains_ currently supported by the Linux kernel, along with the range of personalities they support.
// 
// 0-0   Linux           [kernel]
// 
// Think of execution domains as the "personality" for an operating system. Because other binary formats, such as Solaris, UnixWare, and FreeBSD, can be used with Linux, programmers can change the way the operating system treats system calls from these binaries by changing the personality of the task. Except for the `PER_LINUX` execution domain, different personalities can be implemented as dynamically loadable modules.
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#idm139745909615040


define_struct! {
    pub struct ExecDomains(String);
}

use std::str::FromStr;
impl FromStr for ExecDomains {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<ExecDomains, crate::ProcErr> {
        Ok(ExecDomains(s.to_string()))
    }
}

instance_impl! {
    execdomains, "/proc/execdomains", ExecDomains
}