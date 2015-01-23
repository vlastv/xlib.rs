// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use std::ptr::null;

use libc::{
  c_int,
  c_long,
  c_uint,
  c_ulong,
};

use ::display::Xid;
use ::internal::{
  FieldMask,
  FromNative,
  ToNative
};

/** Visual resource identifier type. */
pub type VisualId = Xid;


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


//
// VisualClass
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum VisualClass {
  StaticGray,
  GrayScale,
  StaticColor,
  PseudoColor,
  TrueColor,
  DirectColor,
}

impl FromNative<c_int> for Option<VisualClass> {
  fn from_native (num: c_int) -> Option<VisualClass> {
    match num {
      0 => Some(VisualClass::StaticGray),
      1 => Some(VisualClass::GrayScale),
      2 => Some(VisualClass::StaticColor),
      3 => Some(VisualClass::PseudoColor),
      4 => Some(VisualClass::TrueColor),
      5 => Some(VisualClass::DirectColor),
      _ => None,
    }
  }
}

impl ToNative<c_int> for VisualClass {
  fn to_native (&self) -> c_int {
    match *self {
      VisualClass::StaticGray => 0,
      VisualClass::GrayScale => 1,
      VisualClass::StaticColor => 2,
      VisualClass::PseudoColor => 3,
      VisualClass::TrueColor => 4,
      VisualClass::DirectColor => 5,
    }
  }
}


//
// VisualInfo
//


#[derive(Clone, Copy)]
pub struct VisualInfo {
  pub visual: Visual,
  pub visual_id: VisualId,
  pub screen: i32,
  pub depth: i32,
  pub class: VisualClass,
  pub red_mask: u32,
  pub green_mask: u32,
  pub blue_mask: u32,
  pub colormap_size: i32,
  pub bits_per_rgb: i32,
}

impl FromNative<::ffi::XVisualInfo> for Option<VisualInfo> {
  fn from_native (xvinfo: ::ffi::XVisualInfo) -> Option<VisualInfo> {
    let vinfo = VisualInfo {
      visual: FromNative::from_native(xvinfo.visual),
      visual_id: xvinfo.visualid as VisualId,
      screen: xvinfo.screen as i32,
      depth: xvinfo.depth as i32,
      class: if let Some(c) = FromNative::from_native(xvinfo.class) {c} else { return None; },
      red_mask: xvinfo.red_mask as u32,
      green_mask: xvinfo.green_mask as u32,
      blue_mask: xvinfo.blue_mask as u32,
      colormap_size: xvinfo.colormap_size as i32,
      bits_per_rgb: xvinfo.bits_per_rgb as i32,
    };
    return Some(vinfo);
  }
}


//
// VisualTemplate
//


#[derive(Clone, Copy, Default)]
pub struct VisualTemplate {
  pub visual_id: Option<VisualId>,
  pub screen: Option<i32>,
  pub depth: Option<i32>,
  pub class: Option<VisualClass>,
  pub red_mask: Option<u32>,
  pub green_mask: Option<u32>,
  pub blue_mask: Option<u32>,
  pub colormap_size: Option<i32>,
  pub bits_per_rgb: Option<i32>,
}

impl FieldMask<c_long> for VisualTemplate {
  fn field_mask (&self) -> c_long {
    let mut mask: c_long = 0;
    if let Some(_) = self.visual_id { mask |= 0x0001; }
    if let Some(_) = self.screen { mask |= 0x0002; }
    if let Some(_) = self.depth { mask |= 0x0004; }
    if let Some(_) = self.class { mask |= 0x0008; }
    if let Some(_) = self.red_mask { mask |= 0x0010; }
    if let Some(_) = self.green_mask { mask |= 0x0020; }
    if let Some(_) = self.blue_mask { mask |= 0x0040; }
    if let Some(_) = self.colormap_size { mask |= 0x0080; }
    if let Some(_) = self.bits_per_rgb { mask |= 0x0100; }
    return mask;
  }
}

impl ToNative<::ffi::XVisualInfo> for VisualTemplate {
  fn to_native (&self) -> ::ffi::XVisualInfo {
    ::ffi::XVisualInfo {
      visual: null(),
      visualid: if let Some(id) = self.visual_id {id as c_ulong} else {0},
      screen: if let Some(n) = self.screen {n as c_int} else {0},
      depth: if let Some(d) = self.depth {d as c_uint} else {0},
      class: if let Some(c) = self.class {c.to_native()} else {0},
      red_mask: if let Some(m) = self.red_mask {m as c_ulong} else {0},
      green_mask: if let Some(m) = self.green_mask {m as c_ulong} else {0},
      blue_mask: if let Some(m) = self.blue_mask {m as c_ulong} else {0},
      colormap_size: if let Some(n) = self.colormap_size {n as c_int} else {0},
      bits_per_rgb: if let Some(n) = self.bits_per_rgb {n as c_int} else {0},
    }
  }
}


//
// public functions
//


pub fn visual_id_from_visual (visual: Visual) -> VisualId {
  unsafe {
    let visual_ptr = visual.to_native();
    if visual_ptr == null() {
      return 0;
    } else {
      return ::ffi::XVisualIDFromVisual(visual_ptr) as VisualId;
    }
  }
}
