pub use composint_macro::composite_type;

#[macro_export]
macro_rules! signed_composint {
    (struct $ident:ident => $($v:ident: $size:literal),+ $(,)?) => {
        $crate::composite_type! {
            struct $ident {
                $($v: signed $size,)+
            }
        }
    };
}

#[macro_export]
macro_rules! unsigned_composint {
    (struct $ident:ident => $($v:ident: $size:literal),+ $(,)?) => {
        $crate::composite_type! {
            struct $ident {
                $($v: unsigned $size,)+
            }
        }
    };
}
