#![warn(clippy::all, rust_2018_idioms)]

/// Central UI elements.
pub(crate) mod central;

/// Common content repeated across multiple pages.
pub(crate) mod common;

/// [`Navbar`] and elements.
pub(crate) mod navbar;
pub(crate) use navbar::Navbar;

/// The main [`Website`] [`eframe::App`]
mod app;
pub use app::Website;
