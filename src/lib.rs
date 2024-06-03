#![warn(clippy::all, rust_2018_idioms)]

/// Central UI elements.
pub(crate) mod central;

/// Gallery of images for Photography section.
pub(crate) mod gallery;

/// Blog posts.
pub(crate) mod blog;

/// Common content repeated across multiple pages.
pub(crate) mod common;

/// [`Navbar`] and elements.
pub(crate) mod navbar;
pub(crate) use navbar::Navbar;

/// The main [`Website`] [`eframe::App`]
mod app;
pub use app::Website;
