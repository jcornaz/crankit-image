use core::{
    ffi::{c_char, CStr},
    fmt::Display,
    ptr,
};

use alloc::{ffi::CString, string::String};
use playdate_sys_v02::ffi::{playdate_graphics, LCDBitmap, LCDBitmapDrawMode, LCDBitmapFlip};

use crate::{DrawImage, DrawMode, HasSize, LoadImage};

pub struct Image<'a> {
    api: &'a playdate_graphics,
    ptr: *mut LCDBitmap,
}

impl<'a> HasSize for Image<'a> {
    fn size(&self) -> [i32; 2] {
        let mut size = [0; 2];
        let mut row_bytes = 0;
        let mut mask: *mut u8 = ptr::null_mut();
        let mut data: *mut u8 = ptr::null_mut();
        unsafe {
            self.api.getBitmapData.unwrap()(
                self.ptr,
                ptr::addr_of_mut!(size[0]),
                ptr::addr_of_mut!(size[1]),
                ptr::addr_of_mut!(row_bytes),
                ptr::addr_of_mut!(mask),
                ptr::addr_of_mut!(data),
            );
        }
        size
    }
}

impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        unsafe { self.api.freeBitmap.unwrap()(self.ptr) }
    }
}

/// Error returned when attempting to load an image that cannot be found
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ImageNotFoundError(String);

impl Display for ImageNotFoundError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Image not found: {}", self.0)
    }
}

#[cfg(feature = "anyhow")]
impl From<ImageNotFoundError> for anyhow::Error {
    fn from(value: ImageNotFoundError) -> Self {
        Self::msg(value.0)
    }
}

impl<'a> LoadImage for &'a playdate_graphics {
    type Image = Image<'a>;
    type Error = ImageNotFoundError;

    fn load_from_path(&self, path: impl AsRef<str>) -> Result<Self::Image, Self::Error> {
        let path = path.as_ref();
        let c_path = CString::new(path).map_err(|_| ImageNotFoundError(path.into()))?;
        let mut outerr: *const c_char = ptr::null_mut();
        unsafe {
            let ptr = self.loadBitmap.unwrap()(c_path.as_ptr(), ptr::addr_of_mut!(outerr));
            if !outerr.is_null() {
                drop(CString::from(CStr::from_ptr(outerr)));
            }
            if ptr.is_null() {
                Err(ImageNotFoundError(path.into()))
            } else {
                Ok(Image { api: self, ptr })
            }
        }
    }
}

impl DrawImage<Image<'_>> for playdate_graphics {
    fn draw_with_flip(
        &self,
        image: &Image<'_>,
        top_left: impl Into<[i32; 2]>,
        flip: impl Into<[bool; 2]>,
    ) {
        let [x, y] = top_left.into();
        let flip = lcd_bitmap_flip(flip);
        unsafe { self.drawBitmap.unwrap()(image.ptr, x, y, flip) }
    }
}

fn lcd_bitmap_flip(flip: impl Into<[bool; 2]>) -> LCDBitmapFlip {
    match flip.into() {
        [false, false] => LCDBitmapFlip::kBitmapUnflipped,
        [true, false] => LCDBitmapFlip::kBitmapFlippedX,
        [false, true] => LCDBitmapFlip::kBitmapFlippedY,
        [true, true] => LCDBitmapFlip::kBitmapFlippedXY,
    }
}

impl From<DrawMode> for LCDBitmapDrawMode {
    fn from(value: DrawMode) -> Self {
        match value {
            DrawMode::Copy => LCDBitmapDrawMode::kDrawModeCopy,
            DrawMode::WhiteTransparent => LCDBitmapDrawMode::kDrawModeWhiteTransparent,
            DrawMode::BlackTransparent => LCDBitmapDrawMode::kDrawModeBlackTransparent,
            DrawMode::FillWhite => LCDBitmapDrawMode::kDrawModeFillWhite,
            DrawMode::FillBlack => LCDBitmapDrawMode::kDrawModeFillBlack,
            DrawMode::XOR => LCDBitmapDrawMode::kDrawModeXOR,
            DrawMode::NXOR => LCDBitmapDrawMode::kDrawModeNXOR,
            DrawMode::Inverted => LCDBitmapDrawMode::kDrawModeInverted,
        }
    }
}
