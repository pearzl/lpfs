macro_rules! define_struct {
    (
        $(#[$odoc: meta])*
        pub struct $name: ident {
            $(
                $(#[$idoc: meta])*
                $item_name: ident : $ty: ty,
            )+
        } => {
            $fn_name: ident,
            $path: literal,
            list()
        }
    ) => {
        #[doc="represent the content of "]
        #[doc=$path]
        #[doc="."]
        $(#[$odoc])*
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name {
            $(
                $item_name : $ty,
            )+
        }

        impl $name {
            $(
                $(#[$idoc])*
                pub fn $item_name(&self) -> &$ty {
                    &self.$item_name
                }
            )*
        }

        pub fn $fn_name() -> Result<Vec<$name>, crate::ProcErr> {
            let content = std::fs::read_to_string($path)?;
            let mut ret = vec![];
            for line in content.trim().lines() {
                let v = $name::from_str(line)?;
                ret.push(v);
            }
            Ok(ret)
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
