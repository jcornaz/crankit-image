use core::{
    ffi::{c_char, CStr},
    ptr,
};

use alloc::ffi::CString;
use playdate_sys_v02::ffi::{playdate_graphics, LCDBitmap};

use crate::{ImageNotFoundError, LoadImage};

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
