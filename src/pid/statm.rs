// /proc/[pid]/statm
// Provides information about memory usage, measured in pages.
// The columns are:
//
//     size       (1) total program size
//                (same as VmSize in /proc/[pid]/status)
//     resident   (2) resident set size
//                (same as VmRSS in /proc/[pid]/status)
//     shared     (3) number of resident shared pages (i.e., backed by a file)
//                (same as RssFile+RssShmem in /proc/[pid]/status)
//     text       (4) text (code)
//     lib        (5) library (unused since Linux 2.6; always 0)
//     data       (6) data + stack
//     dt         (7) dirty pages (unused since Linux 2.6; always 0)
//
// -- http://man7.org/linux/man-pages/man5/proc.5.html
//
//
// statm — The status of the memory in use by the process. Below is a sample /proc/statm file:
// 263 210 210 5 0 205 0
// The seven columns relate to different memory statistics for the process. From left to right, they report the following aspects of the memory used:
// Total program size, in kilobytes.
// Size of memory portions, in kilobytes.
// Number of pages that are shared.
// Number of pages that are code.
// Number of pages of data/stack.
// Number of library pages.
// Number of dirty pages.
//
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-directories#s2-proc-processdirs

define_struct! {
    pub struct Statm {
        size    : usize,
        resident: usize,
        shared  : usize,
        text    : usize,
        lib     : usize,
        data    : usize,
        dt      : usize,
    }
}

impl std::str::FromStr for Statm {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split_ascii_whitespace();
        let size = s_iter.next()
            .ok_or_else(|| Self::Err::from("size not found"))?
            .parse::<usize>()?;
        let resident = s_iter.next()
            .ok_or_else(|| Self::Err::from("resident not found"))?
            .parse::<usize>()?;
        let shared = s_iter.next()
            .ok_or_else(|| Self::Err::from("shared not found"))?
            .parse::<usize>()?;
        let text = s_iter.next()
            .ok_or_else(|| Self::Err::from("text not found"))?
            .parse::<usize>()?;
        let lib = s_iter.next()
            .ok_or_else(|| Self::Err::from("lib not found"))?
            .parse::<usize>()?;
        let data = s_iter.next()
            .ok_or_else(|| Self::Err::from("data not found"))?
            .parse::<usize>()?;
        let dt = s_iter.next()
            .ok_or_else(|| Self::Err::from("dt not found"))?
            .parse::<usize>()?;
        Ok(Statm {
            size    ,
            resident,
            shared  ,
            text    ,
            lib     ,
            data    ,
            dt      ,
        })
    } 
}

pid_instance_impl!{
    statm_of, "statm", Statm,
    statm_self, statm_of_of, statm_self_of, statm_self_self
}
