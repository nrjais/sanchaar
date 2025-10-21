#[cfg(feature = "default")]
pub use local::*;
#[cfg(not(feature = "default"))]
pub use release::*;

#[cfg(feature = "default")]
mod local {
    pub const DELAY: u64 = 5;
    pub const UPDATE_CHECK: bool = false;
}

#[cfg(not(feature = "default"))]
mod release {
    pub const DELAY: u64 = 1;
    pub const UPDATE_CHECK: bool = true;
}
