//! > /proc/loadavg
//! >
//! > The first three fields in this file are load average figures
//! > giving the number of jobs in the run queue (state R) or wait‐
//! > ing for disk I/O (state D) averaged over 1, 5, and 15 minutes.
//! > They are the same as the load average numbers given by
//! > uptime(1) and other programs.  The fourth field consists of
//! > two numbers separated by a slash (/).  The first of these is
//! > the number of currently runnable kernel scheduling entities
//! > (processes, threads).  The value after the slash is the number
//! > of kernel scheduling entities that currently exist on the sys‐
//! > tem.  The fifth field is the PID of the process that was most
//! > recently created on the system.
//! > 
//! > -- http://man7.org/linux/man-pages/man5/proc.5.html
//! 
//! 
//! 
//! > ### 5.2.16.  `/proc/loadavg`
//! > 
//! > <a id="idm139745972429728" class="indexterm" href=""></a>
//! > 
//! > This file provides a look at the load average in regard to both the CPU and IO over time, as well as additional data used by `uptime` and other commands. A sample `/proc/loadavg` file looks similar to the following:
//! > 
//! > <pre class="screen">0.20 0.18 0.12 1/80 11206</pre>
//! > 
//! > The first three columns measure CPU and IO utilization of the last one, five, and 15 minute periods. The fourth column shows the number of currently running processes and the total number of processes. The last column displays the last process ID used.
//! > 
//! > In addition, load average also refers to the number of processes ready to run (i.e. in the run queue, waiting for a CPU share.
//! >
//! > -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-loadavg
//! 

define_struct! {
    /// Represent the content of /proc/loadavg
    /// 
    /// The fields of this struct reference to [`fs/proc/loadavg.c`](https://github.com/torvalds/linux/blob/345671ea0f9258f410eb057b9ced9cefbbe5dc78/fs/proc/loadavg.c).
    pub struct LoadAvg {
        one: f32,
        five: f32,
        fifteen: f32,
        /// The number before the slash(/).
        running_num: i64,
        /// The number after the slash(/).
        total_num: i64,
        latest_pid: i32,
    }
}

use std::str::FromStr;
impl FromStr for LoadAvg {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<LoadAvg, crate::ProcErr> {
        let columns: Vec<&str> = s.split(|c| c == ' ' || c == '/').collect();
        if columns.len() != 6 {
            let emsg = format!("LoadAvg has 5 fields but got {}", columns.len());
            return Err(bfe!(emsg))
        }
        
        let one = columns[0].parse::<f32>()?;
        let five = columns[1].parse::<f32>()?;
        let fifteen = columns[2].parse::<f32>()?;
        let running_num = columns[3].parse::<i64>()?;
        let total_num = columns[4].parse::<i64>()?;
        let latest_pid = columns[5].parse::<i32>()?;

        Ok(LoadAvg{
            one, five, fifteen, running_num,
            total_num, latest_pid
        })
    }
}

instance_impl! {
    loadavg, "/proc/loadavg", LoadAvg
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_loadavg() {
        let source = "0.00 0.03 0.05 1/248 19480";
        let correct = LoadAvg{
            one: 0.00f32,
            five: 0.03f32,
            fifteen: 0.05f32,
            running_num: 1,
            total_num: 248,
            latest_pid: 19480
        };
        assert_eq!(correct, source.parse().unwrap());
    }
}