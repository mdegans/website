#![warn(clippy::all, rust_2018_idioms)]

pub(crate) mod navbar;
pub(crate) use navbar::Navbar;

mod app;
pub use app::Website;
