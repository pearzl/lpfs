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
    };
    (
        $(#[$odoc: meta])*
        pub struct $name: ident ($ty: ty);
    ) => {
        $(#[$odoc])*
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name($ty);

        impl $name {
            pub fn into_inner(self) -> $ty {
                self.0
            }
        }

        impl std::convert::AsRef<$ty> for $name {
            fn as_ref(&self) -> &$ty {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}

macro_rules! list_impl {
    (
        $(#[$k: meta])*
        $fn_name: ident, $path: expr, $return_type: ty, $sep: expr, $skip: literal
    ) => {
        #[doc="Return parsed content of "]
        #[doc=$path]
        #[doc=".\n\n See it's return type for details."]
        $(#[$k])*
        pub fn $fn_name() -> Result<Vec<$return_type>, crate::ProcErr> {
            let content = std::fs::read_to_string($path)?;
            let mut ret = vec![];
            for block in content.trim().split($sep).skip($skip) {
                let v: $return_type = block.parse()?;
                ret.push(v);
            }
            Ok(ret)
        }

        test_impl!($fn_name);
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

        test_impl!($fn_name);
    }
}

macro_rules! pid_instance_impl {
    (
        $(#[$k: meta])*
        $name_of: ident, $file_name: expr, $return_type: ty,
        $name_self: ident, $name_of_of: ident, $name_self_of: ident, $name_self_self: ident
    ) => {
        #[doc="Return parsed content of `/proc/[pid]/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details."]
        $(#[$k])*
        pub fn $name_of(pid: $crate::pid::Pid) -> Result<$return_type, crate::ProcErr> {
            let path = format!(concat!("/proc/{}/", $file_name), pid);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }

        #[doc="Return parsed content of `/proc/self/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details.\n\n"]
        $(#[$k])*
        pub fn $name_self() -> Result<$return_type, crate::ProcErr> {
            let path = concat!("/proc/self/", $file_name);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }

        test_impl!($name_self);

        #[doc="Return parsed content of `/proc/[pid]/task/[tid]/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details.\n\n"]
        $(#[$k])*
        pub fn $name_of_of(pid: $crate::pid::Pid, tid: $crate::pid::Tid) -> Result<$return_type, crate::ProcErr> {
            let path = format!(concat!("/proc/{}/task/{}/", $file_name), pid, tid);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }

        #[doc="Return parsed content of `/proc/self/task/[tid]/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details.\n\n"]
        $(#[$k])*
        pub fn $name_self_of(tid: $crate::pid::Tid) -> Result<$return_type, crate::ProcErr> {
            let path = format!(concat!("/proc/self/task/{}/", $file_name), tid);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }

        #[doc="Return parsed content of `/proc/thread-self/"]
        #[doc=$file_name]
        #[doc="`.\n\n See it's return type for details.\n\n"]
        $(#[$k])*
        pub fn $name_self_self() -> Result<$return_type, crate::ProcErr> {
            let path = concat!("/proc/thread-self/{}", $file_name);
            let content = std::fs::read_to_string(path)?;
            content.trim().parse()
        }
    }
}

macro_rules! test_impl {
    ($fn_name: ident) => {
        #[cfg(test)]
        #[test]
        fn test_impl() {
            use std::io::ErrorKind;

            let ret = $fn_name();
            if let Err(e) = ret {
                match e {
                    $crate::ProcErr::IO(inner_err) => match inner_err.kind() {
                        ErrorKind::NotFound => (),
                        _ => Err(inner_err).unwrap(),
                    },
                    _ => Err(e).unwrap(),
                }
            }
        }
    };
}
