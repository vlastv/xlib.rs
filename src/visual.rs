// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use ::internal::{
  FromNative,
  ToNative
};


//
// Visual
//


#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Visual {
  ptr: *const ::ffi::Visual,
}

impl FromNative<*const ::ffi::Visual> for Visual {
  fn from_native (ptr: *const ::ffi::Visual) -> Visual {
    Visual {
      ptr: ptr,
    }
  }
}

impl ToNative<*const ::ffi::Visual> for Visual {
  fn to_native (&self) -> *const ::ffi::Visual {
    self.ptr
  }
}
