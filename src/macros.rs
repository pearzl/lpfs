macro_rules! define_struct {
    (
        $(#[$odoc: meta])*
        pub struct $name: ident $(<$lt: tt>)? {
            $(
                $(#[$idoc: meta])*
                $item_name: ident : $ty: ty,
            )+
        }
    ) => {
        $(#[$odoc])*
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name $(<$lt>)? {
            $(
                $item_name : $ty,
            )+
        }

        impl$(<$lt>)? $name$(<$lt>)? {
            $(
                $(#[$idoc])*
                pub fn $item_name(&self) -> &$ty {
                    &self.$item_name
                }
            )*
        }
    }
}

macro_rules! list_impl {
    (
        $(#[$k: meta])*
        $fn_name: ident, $path: expr, $return_type: ty, $sep: expr, $skip: literal
    ) => {
        $(#[$k])*
        pub fn $fn_name() -> Result<Vec<$return_type>, crate::ProcErr> {
            let content = std::fs::read_to_string($path)?;
            let mut ret = vec![];
            for line in content.trim().lines() {
                let v: $return_type = line.parse()?;
                ret.push(v);
            }
            Ok(ret)
        }
    }
}

macro_rules! instance_impl {
    (
        $(#[$k: meta])*
        $fn_name: ident, $path: expr, $return_type: ty
    ) => {
        #[doc="Return parsed content of "]
        #[doc=$path]
        #[doc=".\n\n See it's return type for details."]
        $(#[$k])*
        pub fn $fn_name() -> Result<$return_type, crate::ProcErr> {
            let content = std::fs::read_to_string($path)?;
            content.trim().parse()
        }
    }
}

macro_rules! pid_instance_impl {
    (
        $(#[$k: meta])*
        $fn_name: ident, $file_name: expr, $return_type: ty,
        $self_fn_name: ident, $of_task_fn_name: ident, $self_task_fn_name: ident
    ) => {
        #[doc="Return parsed content of `/proc/[pid]/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details."]
        $(#[$k])*
        pub fn $fn_name(pid: u32) -> Result<$return_type, crate::ProcErr> {
            let path = format!(concat!("/proc/{}/", $file_name), pid);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }

        #[doc="Return parsed content of `/proc/self/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details.\n\n"]
        $(#[$k])*
        pub fn $self_fn_name() -> Result<$return_type, crate::ProcErr> {
            let path = concat!("/proc/self/", $file_name);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }

        #[doc="Return parsed content of `/proc/[pid]/task/[tid]/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details.\n\n"]
        $(#[$k])*
        #[cfg(feature = "pid_task" )]
        pub fn $of_task_fn_name(pid: u32, tid: u32) -> Result<$return_type, crate::ProcErr> {
            let path = format!(concat!("/proc/{}/task/{}/", $file_name), pid, tid);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }

        #[doc="Return parsed content of `/proc/self/task/[tid]/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details.\n\n"]
        $(#[$k])*
        #[cfg(feature = "pid_task" )]
        pub fn $self_task_fn_name(tid: u32) -> Result<$return_type, crate::ProcErr> {
            let path = format!(concat!("/proc/self/task/{}/", $file_name), tid);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }
    }
}

macro_rules! define_modules {
    (
        $(
            $mod_name: ident $feature_name: expr;
        )*
    ) => {
        $(
            #[cfg(feature = $feature_name )]
            #[doc(hidden)]
            pub mod $mod_name;
            #[doc(inline)]
            #[cfg(feature = $feature_name )]
            pub use $mod_name::*;
        )*
    };
}

macro_rules! bfe {
    (
        $msg: expr
    ) => {
        crate::ProcErr::BadFormat($msg)
    };
}
