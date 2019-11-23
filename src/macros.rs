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

// macro_rules! list_impl {
//     (
//         $(#[$k: meta])*
//         $fn_name: ident, $path: expr, $return_type: ty, $sep: expr, $skip: literal
//     ) => {
//         $(#[$k])*
//         pub fn $fn_name() -> Result<Vec<$return_type>, crate::ProcErr> {
//             let content = std::fs::read_to_string($path)?;
//             let mut ret = vec![];
//             for line in content.trim().lines() {
//                 let v = $return_type::from_str(line)?;
//                 ret.push(v);
//             }
//             Ok(ret)
//         }
//     }
// }

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
