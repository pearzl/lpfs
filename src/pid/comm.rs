// /proc/[pid]/comm
// 
// 3.6	/proc/<pid>/comm  & /proc/<pid>/task/<tid>/comm
// --------------------------------------------------------
// These files provide a method to access a tasks comm value. It also allows for
// a task to set its own or one of its thread siblings comm value. The comm value
// is limited in size compared to the cmdline value, so writing anything longer
// then the kernel's TASK_COMM_LEN (currently 16 chars) will result in a truncated
// comm value.
// 
// -- https://www.kernel.org/doc/Documentation/filesystems/proc.txt

define_struct! {
    /// Represent the content of /proc/[pid]/comm, returned by [`comm_of()`](fn.comm_of.html).
    /// 
    /// This is an String wrapper with implement of Deref trait.
    pub struct CommP (String);
}

use std::ops::Deref;
impl Deref for CommP {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

use std::str::FromStr;
impl FromStr for CommP {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<CommP, crate::ProcErr> {
        Ok(CommP(s.to_string()))
    }
}

pid_instance_impl! {
    comm_of, "comm", CommP, 
    comm_self, comm_of_task, comm_self_task
}
