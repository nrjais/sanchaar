#[macro_export(local_inner_macros)]
macro_rules! new_id_type {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; $($rest:tt)* ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[repr(transparent)]
        $vis struct $name(uuid::Uuid);

        impl $name {
            pub const ZERO: Self = Self(uuid::Uuid::nil());

            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn from_string(s: String) -> Self {
                Self(uuid::Uuid::parse_str(&s).unwrap_or(uuid::Uuid::nil()))
            }

            pub fn as_str(&self) -> String {
                self.0.to_string()
            }

            pub fn to_string(&self) -> String {
                self.0.to_string()
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

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.0.to_string().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(Self::from_string(s))
            }
        }

        $crate::new_id_type!($($rest)*);
    };
    () => {}
}
