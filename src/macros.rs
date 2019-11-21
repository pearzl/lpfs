macro_rules! define_struct {
    (
        $(#[$odoc: meta])*
        pub struct $name: ident {
            $(
                $(#[$idoc: meta])*
                $item_name: ident : $ty: ty,
            )+
        } => $path: literal;
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
