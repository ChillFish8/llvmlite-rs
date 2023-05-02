use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::ptr;

use llvmlite_types::LLVMContextRef;

use crate::binding::LLVMMethods;

/// A UTF-8 string allocated by LLVM.
///
/// The string lives for as long as the DLL is loaded or until
/// the value is dropped.
pub struct Utf8String<'lib> {
    pub(crate) inner: &'static str,
    pub(crate) ptr: *const c_char,
    pub(crate) lib: LLVMMethods<'lib>,
}

impl<'lib> Utf8String<'lib> {
    pub(crate) fn empty(lib: LLVMMethods<'lib>) -> Self {
        Self {
            inner: "",
            ptr: ptr::null(),
            lib,
        }
    }
}

impl<'lib> Display for Utf8String<'lib> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'lib> Debug for Utf8String<'lib> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<'lib> AsRef<str> for Utf8String<'lib> {
    fn as_ref(&self) -> &str {
        self.inner
    }
}

impl<'lib> Deref for Utf8String<'lib> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'lib> Drop for Utf8String<'lib> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { (self.lib.dispose_string)(self.ptr) };
            self.ptr = ptr::null();
            self.inner = "";
        }
    }
}


/// A non-utf-8 byte string allocated by LLVM.
///
/// The string lives for as long as the DLL is loaded or until
/// the value is dropped.
pub struct ByteString<'lib> {
    pub(crate) inner: Option<&'static CStr>,
    pub(crate) ptr: *const c_char,
    pub(crate) lib: LLVMMethods<'lib>,
}

impl<'lib> ByteString<'lib> {
    pub(crate) fn empty(lib: LLVMMethods<'lib>) -> Self {
        Self {
            inner: None,
            ptr: ptr::null(),
            lib: lib.clone(),
        }
    }
}

impl<'lib> Debug for ByteString<'lib> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<'lib> AsRef<CStr> for ByteString<'lib> {
    fn as_ref(&self) -> &CStr {
        self.inner
            .as_ref()
            .expect("CStr should not be valid unless the pointer has become null")
    }
}

impl<'lib> Deref for ByteString<'lib> {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        self.inner
            .as_ref()
            .expect("CStr should not be valid unless the pointer has become null")
    }
}

impl<'lib> Drop for ByteString<'lib> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { (self.lib.dispose_string)(self.ptr) };
            self.ptr = ptr::null();
            self.inner = None;
        }
    }
}


/// A safe wrapper around a LLVM context.
pub struct Context<'lib> {
    pub(crate) is_global: bool,
    pub(crate) inner: LLVMContextRef,
    pub(crate) lib: LLVMMethods<'lib>,
}

impl<'lib> Context<'lib> {

}

impl<'lib> Drop for Context<'lib> {
    fn drop(&mut self) {
        if !self.inner.is_null() && !self.is_global {
            unsafe { (self.lib.context_dispose)(self.inner) }
        }
    }
}
