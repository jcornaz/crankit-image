#![no_std]

//! A safe and ergonomic image API for the playdate
//!
//! ## Feature flags
//!
//! * `playdate-sys-v02`: implementations of the input source traits for the types `&ffi::playdate_graphics` of the crate [`playdate-sys`](https://docs.rs/playdate-sys/0.2) (version `0.2`)
//! * `anyhow`: implementations of `From` error type for `anyhow::Error`

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
        self.draw_with_flip(image, top_left, [false, false]);
    }

    /// Draw the image on screen with the top-left corner at the given screen coordinates
    fn draw_with_flip(&self, image: &I, top_left: impl Into<[i32; 2]>, flip: impl Into<[bool; 2]>);
}

/// Ability to draw an image from its origin point (instead of from the top-left)
///
/// This trait is automatically implemented for implementations of `DrawImage<I>` where `I: HasSize`
pub trait DrawFromOrigin<I> {
    /// Draw the image so that the `origin` is at `position`
    ///
    /// The origin is expressed in ratio of the size. So `[0., 0.]` is the top-left and `[1.,1.]` is the bottom right.
    fn draw_from_origin(
        &self,
        image: &I,
        position: impl Into<[i32; 2]>,
        origin: impl Into<[f32; 2]>,
    ) {
        self.draw_from_origin_with_flip(image, position, origin, [false, false]);
    }

    /// Draw the image so that the `origin` is at `position` with given `flip` argument.
    ///
    /// The origin is expressed in ratio of the size. So `[0., 0.]` is the top-left and `[1.,1.]` is the bottom right.
    ///
    /// If flipped, the image is fliped around its origin.
    fn draw_from_origin_with_flip(
        &self,
        image: &I,
        position: impl Into<[i32; 2]>,
        origin: impl Into<[f32; 2]>,
        flip: impl Into<[bool; 2]>,
    );
}

impl<T, I> DrawFromOrigin<I> for T
where
    T: DrawImage<I>,
    I: HasSize,
{
    fn draw_from_origin_with_flip(
        &self,
        image: &I,
        position: impl Into<[i32; 2]>,
        origin: impl Into<[f32; 2]>,
        flip: impl Into<[bool; 2]>,
    ) {
        let flip = flip.into();
        let position = position_from_origin(position.into(), origin.into(), image.size(), flip);
        self.draw_with_flip(image, position, flip);
    }
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn position_from_origin(
    [x, y]: [i32; 2],
    [origin_x, origin_y]: [f32; 2],
    [w, h]: [i32; 2],
    [flip_x, flip_y]: [bool; 2],
) -> [i32; 2] {
    let x = if flip_x {
        x - (w as f32 * (1.0 - origin_x)) as i32
    } else {
        x - (w as f32 * origin_x) as i32
    };
    let y = if flip_y {
        y - (h as f32 * (1.0 - origin_y)) as i32
    } else {
        y - (h as f32 * origin_y) as i32
    };
    [x, y]
}

pub trait HasSize {
    fn size(&self) -> [i32; 2];
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0, 0], [0., 0.], [2, 3], [false, false], [0, 0])]
    #[case([0, 0], [1., 0.], [2, 3], [false, false], [-2, 0])]
    #[case([0, 0], [0., 1.], [2, 3], [false, false], [0, -3])]
    #[case([0, 0], [1., 1.], [2, 3], [false, false], [-2, -3])]
    #[case([0, 0], [2., 2.], [2, 3], [false, false], [-4, -6])]
    #[case([0, 0], [0., 0.], [2, 3], [true, false], [-2, 0])]
    #[case([0, 0], [1., 0.], [2, 3], [true, false], [0, 0])]
    #[case([0, 0], [0., 1.], [2, 3], [false, true], [0, 0])]
    #[case([0, 0], [1., 1.], [2, 3], [true, true], [0, 0])]
    #[case([0, 0], [2., 0.], [2, 3], [true, false], [2, 0])]
    #[case([0, 0], [0., 2.], [2, 3], [false, true], [0, 3])]
    fn test_position_from_origin(
        #[case] position: [i32; 2],
        #[case] origin: [f32; 2],
        #[case] size: [i32; 2],
        #[case] flip: [bool; 2],
        #[case] expected: [i32; 2],
    ) {
        let actual = position_from_origin(position, origin, size, flip);
        assert_eq!(actual, expected);
    }
}
