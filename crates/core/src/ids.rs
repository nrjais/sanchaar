#[macro_export(local_inner_macros)]
macro_rules! new_id_type {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; $($rest:tt)* ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[repr(transparent)]
        $vis struct $name(u64);

        impl $name {
            pub const ZERO: Self = Self(0);

            pub fn new() -> Self {
                use std::sync::atomic::{AtomicU64, Ordering};
                static COUNTER: AtomicU64 = AtomicU64::new(1);
                Self(COUNTER.fetch_add(1, Ordering::Relaxed))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::write!(f, "{}", self.0)
            }
        }

        impl std::default::Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        $crate::new_id_type!($($rest)*);
    };
    () => {}
}
