#[cfg(not(feature = "libressl"))]
mod lib;
#[cfg(not(feature = "libressl"))]
pub use lib::*;

extern crate libressl_src;

#[cfg(feature = "libressl")]
pub use libressl_src::*;
