//! Wraps the sass importer interface:
//! https://github.com/sass/libsass/wiki/API-Sass-Importer

use sass_sys;

/// Wrap a Sass Import Callback to control the lifecycle.
pub struct SassImporter {
    pub callback: sass_sys::Sass_C_Import_Callback
}

impl SassImporter {
    pub unsafe fn new(
        arg1: sass_sys::Sass_C_Import_Fn,
        cookie: *mut ::libc::c_void,
    ) -> SassImporter {
        SassImporter {
            callback: sass_sys::sass_make_importer(arg1, cookie)
        }
    }
}


impl Drop for SassImporter {
    fn drop(&mut self) {
        unsafe { sass_sys::sass_delete_importer(self.callback) }
    }
}
