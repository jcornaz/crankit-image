use core::{
    ffi::{c_char, CStr},
    ptr,
};

use alloc::ffi::CString;
use playdate_sys_v02::ffi::{playdate_graphics, LCDBitmap, LCDBitmapFlip};

use crate::{DrawImage, Flip, ImageNotFoundError, LoadImage};

pub struct Image<'a> {
    api: &'a playdate_graphics,
    ptr: *mut LCDBitmap,
}

impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        unsafe { self.api.freeBitmap.unwrap()(self.ptr) }
    }
}

impl<'a> LoadImage for &'a playdate_graphics {
    type Image = Image<'a>;
    type Error = ImageNotFoundError;

    fn load_from_path(&self, path: impl AsRef<str>) -> Result<Self::Image, Self::Error> {
        let path = path.as_ref();
        let c_path = CString::new(path).map_err(|_| ImageNotFoundError::from(path))?;
        let mut outerr: *const c_char = ptr::null_mut();
        unsafe {
            let ptr = self.loadBitmap.unwrap()(c_path.as_ptr(), ptr::addr_of_mut!(outerr));
            if !outerr.is_null() {
                drop(CString::from(CStr::from_ptr(outerr)));
            }
            if ptr.is_null() {
                Err(ImageNotFoundError::from(path))
            } else {
                Ok(Image { api: self, ptr })
            }
        }
    }
}

impl DrawImage<Image<'_>> for playdate_graphics {
    fn draw_with_flip(&self, image: &Image<'_>, top_left: impl Into<[i32; 2]>, flip: Flip) {
        let [x, y] = top_left.into();
        unsafe { self.drawBitmap.unwrap()(image.ptr, x, y, flip.into()) }
    }
}

impl From<Flip> for LCDBitmapFlip {
    fn from(value: Flip) -> Self {
        match value {
            Flip::Unflipped => Self::kBitmapUnflipped,
            Flip::FlippedX => Self::kBitmapFlippedX,
            Flip::FlippedY => Self::kBitmapFlippedY,
            Flip::FlippedXY => Self::kBitmapFlippedXY,
        }
    }
}
