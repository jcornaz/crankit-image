#![no_std]

//! A safe and ergonomic image API for the playdate
//!
//! ## Feature flags
//!
//! * `playdate-sys-v02` (default): provides implementations of the input source traits for the types `ffi::playdate_sys` and `ffi::PlaydateAPI` of the crate [`playdate-sys`](https://docs.rs/playdate-sys/0.2) (version `0.2`)

extern crate alloc;

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

/// Ability to draw an image on screen
pub trait DrawImage<I> {
    /// Draw the image on screen with the top-left corner at the given screen coordinates
    fn draw(&self, image: &I, top_left: impl Into<[i32; 2]>) {
        self.draw_with_flip(image, top_left, Flip::default());
    }

    /// Draw the image on screen with the top-left corner at the given screen coordinates
    fn draw_with_flip(&self, image: &I, top_left: impl Into<[i32; 2]>, flip: Flip);
}

pub trait HasSize {
    fn size(&self) -> [i32; 2];
}

/// Flag indicating if how the image should be flipped
#[allow(clippy::exhaustive_enums)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum Flip {
    /// Do not flip the image
    #[default]
    Unflipped,
    /// Flip horizontaly (on the X axis)
    FlippedX,
    /// Flip verticaly (on the Y axis)
    FlippedY,
    /// Flip both horizontaly and verticaly (on the X and Y axes)
    FlippedXY,
}

#[allow(missing_docs)]
impl Flip {
    #[must_use]
    pub fn new(flip_x: bool, flip_y: bool) -> Self {
        match (flip_x, flip_y) {
            (false, false) => Self::Unflipped,
            (true, false) => Self::FlippedX,
            (false, true) => Self::FlippedY,
            (true, true) => Self::FlippedXY,
        }
    }

    #[must_use]
    pub fn horizontal(flip: bool) -> Self {
        Self::new(flip, false)
    }

    #[must_use]
    pub fn vertical(flip: bool) -> Self {
        Self::new(false, flip)
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum DrawMode {
    /// Images are drawn exactly as they are (black pixels are drawn black and white pixels are drawn white)
    #[default]
    Copy,
    /// Any white portions of an image are drawn transparent (black pixels are drawn black and white pixels are drawn transparent)
    WhiteTransparent,
    /// Any black portions of an image are drawn transparent (black pixels are drawn transparent and white pixels are drawn white)
    BlackTransparent,
    /// All non-transparent pixels are drawn white (black pixels are drawn white and white pixels are drawn white)
    FillWhite,
    /// All non-transparent pixels are drawn black (black pixels are drawn black and white pixels are drawn black)
    FillBlack,
    /// Pixels are drawn inverted on white backgrounds, creating an effect where any white pixels in the original image will always be visible,
    /// regardless of the background color, and any black pixels will appear transparent (on a white background, black pixels are drawn white and white pixels are drawn black)
    XOR,
    /// Pixels are drawn inverted on black backgrounds, creating an effect where any black pixels in the original image will always be visible,
    /// regardless of the background color, and any white pixels will appear transparent (on a black background, black pixels are drawn white and white pixels are drawn black)
    NXOR,
    /// Pixels are drawn inverted (black pixels are drawn white and white pixels are drawn black)
    Inverted,
}
