 

 use std::path::PathBuf;

 type Result<T> = std::result::Result<T, crate::ProcErr>;
 
 /// Return an PathBuf which /proc/self point to.
 ///
 /// The returnd value should be same as `ls -l /proc/self`.
 ///
 /// There is not self() function in this crate, because self is a key word in rust.
 pub fn _self() -> Result<PathBuf> {
     Ok(std::fs::read_link("/proc/self")?)
 }
 
 /// Return the process ID (pid) of calling process.
 /// 
 /// This should have the same output of `(getpid())[http://man7.org/linux/man-pages/man2/getpid.2.html]`,
 /// but it is a safe method.
 /// 
 /// *Note: std::process::id() have same behavior.*
 pub fn self_pid() -> Result<u32> {
     let path = std::fs::read_link("/proc/self")?;
     let pid_str = path.display().to_string();
     let pid = pid_str.parse::<u32>()?;
     Ok(pid)
 }
 
 /// Return a Vector contains thread id whose contained in current process.
 pub fn threads_of() -> Result<Vec<u32>> {
     let dir_entries = std::fs::read_dir("/proc/self/task/")?;
     let mut ret = vec![];
 
     for task_dir in dir_entries {
         let thread_id_str = task_dir?.file_name();
         let thread_id = thread_id_str
             .to_str()
             .ok_or(bfe!("contains non-unicode chatacter".to_string()))?
             .parse::<u32>()?;
         ret.push(thread_id);
     }
 
     Ok(ret)
 }
 
 #[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn test_self_self_pid() {
        println!("/proc/self point to {:?}", _self().unwrap());
        println!("current pid is {:?}", self_pid().unwrap());
    }

    #[test]
    fn test_self_pid() {
        let pid = unsafe{libc::getpid()} as u32;
        assert_eq!(pid, self_pid().unwrap())
    }

 }
 