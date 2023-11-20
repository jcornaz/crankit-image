#![no_std]

//! A safe and ergonomic image API for the playdate
//!
//! ## Feature flags
//!
//! * `playdate-sys-v02` (default): provides implementations of the input source traits for the types `ffi::playdate_sys` and `ffi::PlaydateAPI` of the crate [`playdate-sys`](https://docs.rs/playdate-sys/0.2) (version `0.2`)

extern crate alloc;

use core::fmt::Display;

use alloc::string::String;

/// Implementations of the the API traits
#[allow(missing_docs)]
pub mod impls {

    /// Implementations fpr [playdate-sys](https://docs.rs/playdate-sys) version `0.2`
    #[cfg(feature = "playdate-sys-v02")]
    pub mod playdate_sys_v02;
}

/// Ability to load an image from path
pub trait LoadImage {
    /// Type of image being loaded
    type Image;
    /// Error type representing failure to load an image
    type Error;

    /// Load an image from its path
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the image cannot be loaded (i.e image not found)
    fn load_from_path(&self, path: impl AsRef<str>) -> Result<Self::Image, Self::Error>;
}

/// Error returned when attempting to load an image that cannot be found
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ImageNotFoundError {
    path: String,
}

impl From<String> for ImageNotFoundError {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl From<&str> for ImageNotFoundError {
    fn from(path: &str) -> Self {
        Self { path: path.into() }
    }
}

impl Display for ImageNotFoundError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Image not found: {}", self.path)
    }
}
