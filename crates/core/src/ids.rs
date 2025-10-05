#[macro_export(local_inner_macros)]
macro_rules! new_id_type {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; $($rest:tt)* ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize)]
        #[repr(transparent)]
        #[serde(transparent)]
        $vis struct $name(uuid::Uuid);

        impl $name {
            pub const ZERO: Self = Self(uuid::Uuid::nil());

            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
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
