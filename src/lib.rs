#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects, unsafe_code)]
// To use `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license file, and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]
#![allow(clippy::match_bool)]

mod consts;
mod error;
mod rolls;
mod score;
pub use error::Error;
pub use rolls::Rolls;
type Result<T> = std::result::Result<T, Error>;