//! [GEOS](https://trac.osgeo.org/geos/) C API bindings.
//!
//! It provides C-interface as is. If you want to use a more Rust-friendly crate,
//! prefer to use the [georust/geos](https://github.com/georust/geos) crate.
//!
//! You can also find it on [crates.io](https://crates.io/crates/geos).

extern crate libc;

#[cfg(feature = "static")]
extern crate link_cplusplus;

pub use functions::*;
pub use types::*;

mod functions;
mod types;
