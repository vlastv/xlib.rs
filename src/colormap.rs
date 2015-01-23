// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use libc::{
  c_char,
  c_ushort,
};

use ::display::Xid;
use ::internal::{
  FieldMask,
  ToNative,
};

/** Colormap identifier type. */
pub type Colormap = Xid;


//
// Color
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Color {
  pub red: Option<u16>,
  pub green: Option<u16>,
  pub blue: Option<u16>,
}

impl FieldMask<c_char> for Color {
  fn field_mask (&self) -> c_char {
    let mut mask: c_char = 0;
    if let Some(_) = self.red { mask |= 0x01; }
    if let Some(_) = self.green { mask |= 0x02; }
    if let Some(_) = self.blue { mask |= 0x04; }
    return mask;
  }
}

impl ToNative<::ffi::XColor> for Color {
  fn to_native (&self) -> ::ffi::XColor {
    ::ffi::XColor {
      pixel: 0,
      red: if let Some(n) = self.red {n as c_ushort} else {0},
      green: if let Some(n) = self.green {n as c_ushort} else {0},
      blue: if let Some(n) = self.blue {n as c_ushort} else {0},
      flags: self.field_mask(),
      pad: 0,
    }
  }
}
