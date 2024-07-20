use crate::ffi;
use std::ffi::c_void;
use std::ops::Deref;

pub(crate) struct SoplexPtr {
    ptr: *mut c_void,
}

impl SoplexPtr {
    pub(crate) fn new() -> Self {
        let ptr = unsafe { ffi::SoPlex_create() };
        assert!(!ptr.is_null());
        SoplexPtr { ptr }
    }
}

impl Default for SoplexPtr {
    fn default() -> Self {
        SoplexPtr::new()
    }
}

impl Deref for SoplexPtr {
    type Target = *mut c_void;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl Drop for SoplexPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::SoPlex_free(self.ptr);
        }
    }
}
