// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use ::internal::{
  FromNative,
  ToNative,
};


//
// Screen
//


#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Screen {
  ptr: *const ::ffi::Screen,
}

impl FromNative<*const ::ffi::Screen> for Screen {
  fn from_native (ptr: *const ::ffi::Screen) -> Screen {
    Screen {
      ptr: ptr,
    }
  }
}

impl ToNative<*const ::ffi::Screen> for Screen {
  fn to_native (&self) -> *const ::ffi::Screen {
    self.ptr
  }
}
