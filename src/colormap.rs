// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use libc::{
  c_char,
  c_ulong,
  c_ushort,
};

use ::display::Xid;
use ::internal::{
  FieldMask,
  FromNative,
  ToNative,
};

/** Colormap identifier type. */
pub type Colormap = Xid;


//
// Color
//


#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Color {
  pub pixel: u32,
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

impl FromNative<::ffi::XColor> for Color {
  fn from_native (xcolor: ::ffi::XColor) -> Color {
    Color {
      pixel: xcolor.pixel as u32,
      red: if xcolor.flags & 0x01 == 0 {None} else {Some(xcolor.red as u16)},
      green: if xcolor.flags & 0x02 == 0 {None} else {Some(xcolor.green as u16)},
      blue: if xcolor.flags & 0x04 == 0 {None} else {Some(xcolor.blue as u16)},
    }
  }
}

impl ToNative<::ffi::XColor> for Color {
  fn to_native (&self) -> ::ffi::XColor {
    ::ffi::XColor {
      pixel: self.pixel as c_ulong,
      red: if let Some(n) = self.red {n as c_ushort} else {0},
      green: if let Some(n) = self.green {n as c_ushort} else {0},
      blue: if let Some(n) = self.blue {n as c_ushort} else {0},
      flags: self.field_mask(),
      pad: 0,
    }
  }
}
