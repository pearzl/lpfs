// /proc/[pid]/stat
// 
// Status information about the process.  This is used by ps(1).
// It is defined in the kernel source file fs/proc/array.c.
// 
// The fields, in order, with their proper scanf(3) format speci‐
// fiers, are listed below.  Whether or not certain of these
// fields display valid information is governed by a ptrace
// access mode PTRACE_MODE_READ_FSCREDS | PTRACE_MODE_NOAUDIT
// check (refer to ptrace(2)).  If the check denies access, then
// the field value is displayed as 0.  The affected fields are
// indicated with the marking [PT].
// 
// (1) pid  %d
//           The process ID.
// 
// (2) comm  %s
//           The filename of the executable, in parentheses.
//           This is visible whether or not the executable is
//           swapped out.
// 
// (3) state  %c
//           One of the following characters, indicating process
//           state:
// 
//           R  Running
// 
//           S  Sleeping in an interruptible wait
// 
//           D  Waiting in uninterruptible disk sleep
// 
//           Z  Zombie
// 
//           T  Stopped (on a signal) or (before Linux 2.6.33)
//              trace stopped
// 
//           t  Tracing stop (Linux 2.6.33 onward)
// 
//           W  Paging (only before Linux 2.6.0)
// 
//           X  Dead (from Linux 2.6.0 onward)
// 
//           x  Dead (Linux 2.6.33 to 3.13 only)
// 
//           K  Wakekill (Linux 2.6.33 to 3.13 only)
// 
//           W  Waking (Linux 2.6.33 to 3.13 only)
// 
//           P  Parked (Linux 3.9 to 3.13 only)
// 
// (4) ppid  %d
//           The PID of the parent of this process.
// 
// (5) pgrp  %d
//           The process group ID of the process.
// 
// (6) session  %d
//           The session ID of the process.
// 
// (7) tty_nr  %d
//           The controlling terminal of the process.  (The minor
//           device number is contained in the combination of
//           bits 31 to 20 and 7 to 0; the major device number is
//           in bits 15 to 8.)
// 
// (8) tpgid  %d
//           The ID of the foreground process group of the con‐
//           trolling terminal of the process.
// 
// (9) flags  %u
//           The kernel flags word of the process.  For bit mean‐
//           ings, see the PF_* defines in the Linux kernel
//           source file include/linux/sched.h.  Details depend
//           on the kernel version.
// 
//           The format for this field was %lu before Linux 2.6.
// 
// (10) minflt  %lu
//           The number of minor faults the process has made
//           which have not required loading a memory page from
//           disk.
// 
// (11) cminflt  %lu
//           The number of minor faults that the process's
//           waited-for children have made.
// 
// (12) majflt  %lu
//           The number of major faults the process has made
//           which have required loading a memory page from disk.
// 
// (13) cmajflt  %lu
//           The number of major faults that the process's
//           waited-for children have made.
// 
// (14) utime  %lu
//           Amount of time that this process has been scheduled
//           in user mode, measured in clock ticks (divide by
//           sysconf(_SC_CLK_TCK)).  This includes guest time,
//           guest_time (time spent running a virtual CPU, see
//           below), so that applications that are not aware of
//           the guest time field do not lose that time from
//           their calculations.
// 
// (15) stime  %lu
//           Amount of time that this process has been scheduled
//           in kernel mode, measured in clock ticks (divide by
//           sysconf(_SC_CLK_TCK)).
// 
// (16) cutime  %ld
//           Amount of time that this process's waited-for chil‐
//           dren have been scheduled in user mode, measured in
//           clock ticks (divide by sysconf(_SC_CLK_TCK)).  (See
//           also times(2).)  This includes guest time,
//           cguest_time (time spent running a virtual CPU, see
//           below).
// 
// (17) cstime  %ld
//           Amount of time that this process's waited-for chil‐
//           dren have been scheduled in kernel mode, measured in
//           clock ticks (divide by sysconf(_SC_CLK_TCK)).
// 
// (18) priority  %ld
//           (Explanation for Linux 2.6) For processes running a
//           real-time scheduling policy (policy below; see
//           sched_setscheduler(2)), this is the negated schedul‐
//           ing priority, minus one; that is, a number in the
//           range -2 to -100, corresponding to real-time priori‐
//           ties 1 to 99.  For processes running under a non-
//           real-time scheduling policy, this is the raw nice
//           value (setpriority(2)) as represented in the kernel.
//           The kernel stores nice values as numbers in the
//           range 0 (high) to 39 (low), corresponding to the
//           user-visible nice range of -20 to 19.
// 
//           Before Linux 2.6, this was a scaled value based on
//           the scheduler weighting given to this process.
// 
// (19) nice  %ld
//           The nice value (see setpriority(2)), a value in the
//           range 19 (low priority) to -20 (high priority).
// 
// (20) num_threads  %ld
//           Number of threads in this process (since Linux 2.6).
//           Before kernel 2.6, this field was hard coded to 0 as
//           a placeholder for an earlier removed field.
// 
// (21) itrealvalue  %ld
//           The time in jiffies before the next SIGALRM is sent
//           to the process due to an interval timer.  Since ker‐
//           nel 2.6.17, this field is no longer maintained, and
//           is hard coded as 0.
// 
// (22) starttime  %llu
//           The time the process started after system boot.  In
//           kernels before Linux 2.6, this value was expressed
//           in jiffies.  Since Linux 2.6, the value is expressed
//           in clock ticks (divide by sysconf(_SC_CLK_TCK)).
// 
//           The format for this field was %lu before Linux 2.6.
// 
// (23) vsize  %lu
//           Virtual memory size in bytes.
// 
// (24) rss  %ld
//           Resident Set Size: number of pages the process has
//           in real memory.  This is just the pages which count
//           toward text, data, or stack space.  This does not
//           include pages which have not been demand-loaded in,
//           or which are swapped out.
// 
// (25) rsslim  %lu
//           Current soft limit in bytes on the rss of the
//           process; see the description of RLIMIT_RSS in
//           getrlimit(2).
// 
// (26) startcode  %lu  [PT]
//           The address above which program text can run.
// 
// (27) endcode  %lu  [PT]
//           The address below which program text can run.
// 
// (28) startstack  %lu  [PT]
//           The address of the start (i.e., bottom) of the
//           stack.
// 
// (29) kstkesp  %lu  [PT]
//           The current value of ESP (stack pointer), as found
//           in the kernel stack page for the process.
// 
// (30) kstkeip  %lu  [PT]
//           The current EIP (instruction pointer).
// 
// (31) signal  %lu
//           The bitmap of pending signals, displayed as a deci‐
//           mal number.  Obsolete, because it does not provide
//           information on real-time signals; use
//           /proc/[pid]/status instead.
// 
// (32) blocked  %lu
//           The bitmap of blocked signals, displayed as a deci‐
//           mal number.  Obsolete, because it does not provide
//           information on real-time signals; use
//           /proc/[pid]/status instead.
// 
// (33) sigignore  %lu
//           The bitmap of ignored signals, displayed as a deci‐
//           mal number.  Obsolete, because it does not provide
//           information on real-time signals; use
//           /proc/[pid]/status instead.
// 
// (34) sigcatch  %lu
//           The bitmap of caught signals, displayed as a decimal
//           number.  Obsolete, because it does not provide
//           information on real-time signals; use
//           /proc/[pid]/status instead.
// 
// (35) wchan  %lu  [PT]
//           This is the "channel" in which the process is wait‐
//           ing.  It is the address of a location in the kernel
//           where the process is sleeping.  The corresponding
//           symbolic name can be found in /proc/[pid]/wchan.
// 
// (36) nswap  %lu
//           Number of pages swapped (not maintained).
// 
// (37) cnswap  %lu
//           Cumulative nswap for child processes (not main‐
//           tained).
// 
// (38) exit_signal  %d  (since Linux 2.1.22)
//           Signal to be sent to parent when we die.
// 
// (39) processor  %d  (since Linux 2.2.8)
//           CPU number last executed on.
// 
// (40) rt_priority  %u  (since Linux 2.5.19)
//           Real-time scheduling priority, a number in the range
//           1 to 99 for processes scheduled under a real-time
//           policy, or 0, for non-real-time processes (see
//           sched_setscheduler(2)).
// 
// (41) policy  %u  (since Linux 2.5.19)
//           Scheduling policy (see sched_setscheduler(2)).
//           Decode using the SCHED_* constants in linux/sched.h.
// 
//           The format for this field was %lu before Linux
//           2.6.22.
// 
// (42) delayacct_blkio_ticks  %llu  (since Linux 2.6.18)
//           Aggregated block I/O delays, measured in clock ticks
//           (centiseconds).
// 
// (43) guest_time  %lu  (since Linux 2.6.24)
//           Guest time of the process (time spent running a vir‐
//           tual CPU for a guest operating system), measured in
//           clock ticks (divide by sysconf(_SC_CLK_TCK)).
// 
// (44) cguest_time  %ld  (since Linux 2.6.24)
//           Guest time of the process's children, measured in
//           clock ticks (divide by sysconf(_SC_CLK_TCK)).
// 
// (45) start_data  %lu  (since Linux 3.3)  [PT]
//           Address above which program initialized and unini‐
//           tialized (BSS) data are placed.
// 
// (46) end_data  %lu  (since Linux 3.3)  [PT]
//           Address below which program initialized and unini‐
//           tialized (BSS) data are placed.
// 
// (47) start_brk  %lu  (since Linux 3.3)  [PT]
//           Address above which program heap can be expanded
//           with brk(2).
// 
// (48) arg_start  %lu  (since Linux 3.5)  [PT]
//           Address above which program command-line arguments
//           (argv) are placed.
// 
// (49) arg_end  %lu  (since Linux 3.5)  [PT]
//           Address below program command-line arguments (argv)
//           are placed.
// 
// (50) env_start  %lu  (since Linux 3.5)  [PT]
//           Address above which program environment is placed.
// 
// (51) env_end  %lu  (since Linux 3.5)  [PT]
//           Address below which program environment is placed.
// 
// (52) exit_code  %d  (since Linux 3.5)  [PT]
//           The thread's exit status in the form reported by
//           waitpid(2).
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html

define_struct! {
    /// Represent the content of /proc/[pid]/stat and /proc/[pid]/task/[tid]/stat. 
    /// Returned by [`stat_of()`](fn.stat_of.html).
    /// 
    /// Reference to [`fs/proc/array.c`](https://github.com/torvalds/linux/blob/master/fs/proc/array.c)
    pub struct StatP {
        pid: i32,
        comm: String,
        state: char,
        ppid: i32,
        pgrp: i32,
        session: i32,
        tty_nr: i32,
        tpgid: i32,
        flags: u32,
        minflt: u64,
        cminflt: u64,
        majflt: u64,
        cmajflt: u64,
        utime: u64,
        stime: u64,
        cutime: i64,
        cstime: i64,
        priority: i64,
        nice: i64,
        num_threads: i64,
        itrealvalue: i64,
        starttime: u128,
        vsize: u64,
        rss: i64,
        rsslim: u64,
        startcode: u64,
        endcode: u64,
        startstack: u64,
        kstkesp: u64,
        kstkeip: u64,
        signal: u64,
        blocked: u64,
        sigignore: u64,
        sigcatch: u64,
        wchan: u64,
        nswap: u64,
        cnswap: u64,
        exit_signal: i32,
        processor: i32,
        rt_priority: u32,
        policy: u32,
        delayacct_blkio_ticks: u128,
        guest_time: u64,
        cguest_time: i64,
        start_data: Option<u64>,
        end_data: Option<u64>,
        start_brk: Option<u64>,
        arg_start: Option<u64>,
        arg_end: Option<u64>,
        env_start: Option<u64>,
        env_end: Option<u64>,
        exit_code: Option<i32>,
    }
}

use std::str::FromStr;
impl FromStr for StatP {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<StatP, crate::ProcErr> {
        let columns_: Vec<&str> = s.split(|c| c == '(' || c == ')').collect();
        if columns_.len() != 3 {
            return Err("no enough fields to parse a StatP".into());
        }
        let mut columns: Vec<&str> = vec![columns_[0].trim(), columns_[1].trim()];
        columns.extend(columns_[2].trim().split_ascii_whitespace());

        macro_rules! unwrap_integer {
            (
                $source: expr, $type: ty, $field: ident
            ) => {
                let $field = $source.parse::<$type>()
                    .map_err(|_| concat!("parse ", stringify!($field)," failed"))?;
            }
        }

        macro_rules! unwrap_opt_integer {
            (
                $source: expr, $get_n: expr, $type: ty, $field: ident
            ) => {
                let $field = if let Some(v) = $source.get($get_n) {
                    let value = v.parse::<$type>()
                        .map_err(|_| concat!("parse ", stringify!($field)," failed"))?;
                    Some(value)
                }else {
                    None
                };
            }
        }

        unwrap_integer!(columns[0], i32, pid);
        let comm = columns[1].to_string();
        let state = columns[2].chars().next().ok_or_else(||"stat is empty")?;
        unwrap_integer!(columns[3], i32, ppid);
        unwrap_integer!(columns[4], i32, pgrp);
        unwrap_integer!(columns[5], i32, session);
        unwrap_integer!(columns[6], i32, tty_nr);
        unwrap_integer!(columns[7], i32, tpgid);
        unwrap_integer!(columns[8], u32, flags);
        unwrap_integer!(columns[9], u64, minflt);
        unwrap_integer!(columns[10], u64, cminflt);
        unwrap_integer!(columns[11], u64, majflt);
        unwrap_integer!(columns[12], u64, cmajflt);
        unwrap_integer!(columns[13], u64, utime);
        unwrap_integer!(columns[14], u64, stime);
        unwrap_integer!(columns[15], i64, cutime);
        unwrap_integer!(columns[16], i64, cstime);
        unwrap_integer!(columns[17], i64, priority);
        unwrap_integer!(columns[18], i64, nice);
        unwrap_integer!(columns[19], i64, num_threads);
        unwrap_integer!(columns[20], i64, itrealvalue);
        unwrap_integer!(columns[21], u128, starttime);
        unwrap_integer!(columns[22], u64, vsize);
        unwrap_integer!(columns[23], i64, rss);
        unwrap_integer!(columns[24], u64, rsslim);
        unwrap_integer!(columns[25], u64, startcode);
        unwrap_integer!(columns[26], u64, endcode);
        unwrap_integer!(columns[27], u64, startstack);
        unwrap_integer!(columns[28], u64, kstkesp);
        unwrap_integer!(columns[29], u64, kstkeip);
        unwrap_integer!(columns[30], u64, signal);
        unwrap_integer!(columns[31], u64, blocked);
        unwrap_integer!(columns[32], u64, sigignore);
        unwrap_integer!(columns[33], u64, sigcatch);
        unwrap_integer!(columns[34], u64, wchan);
        unwrap_integer!(columns[35], u64, nswap);
        unwrap_integer!(columns[36], u64, cnswap);
        unwrap_integer!(columns[37], i32, exit_signal);
        unwrap_integer!(columns[38], i32, processor);
        unwrap_integer!(columns[39], u32, rt_priority);
        unwrap_integer!(columns[40], u32, policy);
        unwrap_integer!(columns[41], u128, delayacct_blkio_ticks);
        unwrap_integer!(columns[42], u64, guest_time);
        unwrap_integer!(columns[43], i64, cguest_time);
        unwrap_opt_integer!(columns, 44, u64, start_data);
        unwrap_opt_integer!(columns, 45, u64, end_data);
        unwrap_opt_integer!(columns, 46, u64, start_brk);
        unwrap_opt_integer!(columns, 47, u64, arg_start);
        unwrap_opt_integer!(columns, 48, u64, arg_end);
        unwrap_opt_integer!(columns, 49, u64, env_start);
        unwrap_opt_integer!(columns, 50, u64, env_end);
        unwrap_opt_integer!(columns, 51, i32,  exit_code);

        Ok(StatP{
            pid, comm, state, ppid, pgrp, session, tty_nr, tpgid,
            flags, minflt, cminflt, majflt, cmajflt, utime, stime,
            cutime, cstime, priority, nice, num_threads, itrealvalue,
            starttime, vsize, rss, rsslim, startcode, endcode, startstack,
            kstkesp, kstkeip, signal, blocked, sigignore, sigcatch, wchan,
            nswap, cnswap, exit_signal, processor, rt_priority,
            policy, delayacct_blkio_ticks, guest_time, cguest_time,
            start_data, end_data, start_brk, arg_start, arg_end,
            env_start, env_end, exit_code
        })
    }
}

pid_instance_impl! {
    stat_of, "stat", StatP, 
    stat_self, stat_of_task, stat_self_task
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_statp() {
        let source = "1 (systemd) S 0 1 1 0 -1 4202752 14369183 764090677 397 46639 54116 70774 2865762 770103 20 0 1 0 4 55685120 981 18446744073709551615 94270761308160 94270762758152 140727263428848 140727263425536 140367080586851 0 671173123 4096 1260 18446744072542055742 0 0 17 0 0 0 76 0 0 94270764855672 94270765000248 94270777630720 140727263432620 140727263432671 140727263432671 140727263432671 0";
        let correct = StatP {
            pid: 1,
            comm: String::from("systemd"),
            state: 'S',
            ppid: 0,
            pgrp: 1,
            session: 1,
            tty_nr: 0,
            tpgid: -1,
            flags: 4202752,
            minflt: 14369183,
            cminflt: 764090677,
            majflt: 397,
            cmajflt: 46639,
            utime: 54116,
            stime: 70774,
            cutime: 2865762,
            cstime: 770103,
            priority: 20,
            nice: 0,
            num_threads: 1,
            itrealvalue: 0,
            starttime: 4,
            vsize: 55685120,
            rss: 981,
            rsslim: 18446744073709551615,
            startcode: 94270761308160,
            endcode: 94270762758152,
            startstack: 140727263428848,
            kstkesp: 140727263425536,
            kstkeip: 140367080586851,
            signal: 0,
            blocked: 671173123,
            sigignore: 4096,
            sigcatch: 1260,
            wchan: 18446744072542055742,
            nswap: 0,
            cnswap: 0,
            exit_signal: 17,
            processor: 0,
            rt_priority: 0,
            policy: 0,
            delayacct_blkio_ticks: 76,
            guest_time: 0,
            cguest_time: 0,
            start_data: Some(94270764855672),
            end_data: Some(94270765000248),
            start_brk: Some(94270777630720),
            arg_start: Some(140727263432620),
            arg_end: Some(140727263432671),
            env_start: Some(140727263432671),
            env_end: Some(140727263432671),
            exit_code: Some(0),
        };
        assert_eq!(correct, source.parse::<StatP>().unwrap());

        let source = "1410 (Network File Th) S 251 251 1 0 -1 1077960768 8 2764 0 32 0 0 1 3 20 0 193 0 1541 2565836800 93388 4294967295 3078164480 3078178764 3214281856 2368030232 2999026130 0 4612 4096 1073775868 3223042942 0 0 -1 0 0 3 0 0 0 3078184256 3078184948 3092856832 3214289929 3214290005 3214290005 3214290916 0";
        let correct = StatP {
            pid: 1410,
            comm: String::from("Network File Th"),
            state: 'S',
            ppid: 251,
            pgrp: 251,
            session: 1,
            tty_nr: 0,
            tpgid: -1,
            flags: 1077960768,
            minflt: 8,
            cminflt: 2764,
            majflt: 0,
            cmajflt: 32,
            utime: 0,
            stime: 0,
            cutime: 1,
            cstime: 3,
            priority: 20,
            nice: 0,
            num_threads: 193,
            itrealvalue: 0,
            starttime: 1541,
            vsize: 2565836800,
            rss: 93388,
            rsslim: 4294967295,
            startcode: 3078164480,
            endcode: 3078178764,
            startstack: 3214281856,
            kstkesp: 2368030232,
            kstkeip: 2999026130,
            signal: 0,
            blocked: 4612,
            sigignore: 4096,
            sigcatch: 1073775868,
            wchan: 3223042942,
            nswap: 0,
            cnswap: 0,
            exit_signal: -1,
            processor: 0,
            rt_priority: 0,
            policy: 3,
            delayacct_blkio_ticks: 0,
            guest_time: 0,
            cguest_time: 0,
            start_data: Some(3078184256),
            end_data: Some(3078184948),
            start_brk: Some(3092856832),
            arg_start: Some(3214289929),
            arg_end: Some(3214290005),
            env_start: Some(3214290005),
            env_end: Some(3214290916),
            exit_code: Some(0),
        };
        assert_eq!(correct, source.parse::<StatP>().unwrap());
    }
}
