// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use libc::{
  c_int,
  c_short,
  c_ulong,
  c_ushort,
};

use ::font::Font;
use ::display::Xid;
use ::internal::{
  FieldMask,
  ToNative,
};
use ::pixmap::Pixmap;

/** Graphics context resource identifier type. Renamed from `Gc` to `Gcid` to avoid confusion with `std::gc::Gc`. */
pub type Gcid = Xid;


//
// ArcMode
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ArcMode {
  Chord,
  PieSlice,
}

impl ToNative<c_int> for ArcMode {
  fn to_native (&self) -> c_int {
    match *self {
      ArcMode::Chord => 0,
      ArcMode::PieSlice => 1,
    }
  }
}


//
// CapStyle
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum CapStyle {
  NotLast,
  Butt,
  Round,
  Projecting,
}

impl ToNative<c_int> for CapStyle {
  fn to_native (&self) -> c_int {
    match *self {
      CapStyle::NotLast => 0,
      CapStyle::Butt => 1,
      CapStyle::Round => 2,
      CapStyle::Projecting => 3,
    }
  }
}


//
// ClipOrdering
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ClipOrdering {
  Unsorted,
  YSorted,
  YXSorted,
  YXBanded,
}

impl ToNative<c_int> for ClipOrdering {
  fn to_native (&self) -> c_int {
    match *self {
      ClipOrdering::Unsorted => 0,
      ClipOrdering::YSorted => 1,
      ClipOrdering::YXSorted => 2,
      ClipOrdering::YXBanded => 3,
    }
  }
}


//
// FillRule
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum FillRule {
  EvenOdd,
  Winding,
}

impl ToNative<c_int> for FillRule {
  fn to_native (&self) -> c_int {
    match *self {
      FillRule::EvenOdd => 0,
      FillRule::Winding => 1,
    }
  }
}


//
// FillStyle
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum FillStyle {
  Solid,
  Tiled,
  Stippled,
  OpaqueStippled,
}

impl ToNative<c_int> for FillStyle {
  fn to_native (&self) -> c_int {
    match *self {
      FillStyle::Solid => 0,
      FillStyle::Tiled => 1,
      FillStyle::Stippled => 2,
      FillStyle::OpaqueStippled => 3,
    }
  }
}


//
// GcFunction
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum GcFunction {
  Clear,
  And,
  AndReverse,
  Copy,
  AndInverted,
  Noop,
  Xor,
  Or,
  Nor,
  Equiv,
  Invert,
  OrReverse,
  CopyInverted,
  OrInverted,
  Nand,
  Set,
}

impl ToNative<c_int> for GcFunction {
  fn to_native (&self) -> c_int {
    match *self {
      GcFunction::Clear => 0,
      GcFunction::And => 1,
      GcFunction::AndReverse => 2,
      GcFunction::Copy => 3,
      GcFunction::AndInverted => 4,
      GcFunction::Noop => 5,
      GcFunction::Xor => 6,
      GcFunction::Or => 7,
      GcFunction::Nor => 8,
      GcFunction::Equiv => 9,
      GcFunction::Invert => 10,
      GcFunction::OrReverse => 11,
      GcFunction::CopyInverted => 12,
      GcFunction::OrInverted => 13,
      GcFunction::Nand => 14,
      GcFunction::Set => 15,
    }
  }
}


//
// GcValues
//


#[derive(Clone, Copy, Default)]
pub struct GcValues {
  pub function: Option<GcFunction>,
  pub plane_mask: Option<u32>,
  pub foreground: Option<u32>,
  pub background: Option<u32>,
  pub line_width: Option<i32>,
  pub line_style: Option<LineStyle>,
  pub cap_style: Option<CapStyle>,
  pub join_style: Option<JoinStyle>,
  pub fill_style: Option<FillStyle>,
  pub fill_rule: Option<FillRule>,
  pub arc_mode: Option<ArcMode>,
  pub tile: Option<Pixmap>,
  pub stipple: Option<Pixmap>,
  pub ts_x_origin: Option<i32>,
  pub ts_y_origin: Option<i32>,
  pub font: Option<Font>,
  pub subwindow_mode: Option<SubwindowMode>,
  pub graphics_exposures: Option<bool>,
  pub clip_x_origin: Option<i32>,
  pub clip_y_origin: Option<i32>,
  pub clip_mask: Option<Pixmap>,
  pub dash_offset: Option<i32>,
  // There's also a `dashes` field, but I'm not sure how the hell to use it.
}

impl FieldMask<c_ulong> for GcValues {
  fn field_mask (&self) -> c_ulong {
    let mut mask: c_ulong = 0;
    if let Some(_) = self.function { mask |= 0x0000_0001; }
    if let Some(_) = self.plane_mask { mask |= 0x0000_0002; }
    if let Some(_) = self.foreground { mask |= 0x0000_0004; }
    if let Some(_) = self.background { mask |= 0x0000_0008; }
    if let Some(_) = self.line_width { mask |= 0x0000_0010; }
    if let Some(_) = self.line_style { mask |= 0x0000_0020; }
    if let Some(_) = self.cap_style { mask |= 0x0000_0040; }
    if let Some(_) = self.join_style { mask |= 0x0000_0080; }
    if let Some(_) = self.fill_style { mask |= 0x0000_0100; }
    if let Some(_) = self.fill_rule { mask |= 0x0000_0200; }
    if let Some(_) = self.tile { mask |= 0x0000_0400; }
    if let Some(_) = self.stipple { mask |= 0x0000_0800; }
    if let Some(_) = self.ts_x_origin { mask |= 0x0000_1000; }
    if let Some(_) = self.ts_y_origin { mask |= 0x0000_2000; }
    if let Some(_) = self.font { mask |= 0x0000_4000; }
    if let Some(_) = self.subwindow_mode { mask |= 0x0000_8000; }
    if let Some(_) = self.graphics_exposures { mask |= 0x0001_0000; }
    if let Some(_) = self.clip_x_origin { mask |= 0x0002_0000; }
    if let Some(_) = self.clip_y_origin { mask |= 0x0004_0000; }
    if let Some(_) = self.clip_mask { mask |= 0x0008_0000; }
    if let Some(_) = self.dash_offset { mask |= 0x0010_0000; }
    // TODO: GCDashList = 0x0020_0000
    if let Some(_) = self.arc_mode { mask |= 0x0040_0000; }
    return mask;
  }
}

impl ToNative<::ffi::XGCValues> for GcValues {
  fn to_native (&self) -> ::ffi::XGCValues {
    ::ffi::XGCValues {
      function: if let Some(f) = self.function {f.to_native()} else {0},
      plane_mask: if let Some(m) = self.plane_mask {m as c_ulong} else {0},
      foreground: if let Some(p) = self.foreground {p as c_ulong} else {0},
      background: if let Some(p) = self.background {p as c_ulong} else {0},
      line_width: if let Some(w) = self.line_width {w as c_int} else {0},
      line_style: if let Some(s) = self.line_style {s.to_native()} else {0},
      cap_style: if let Some(s) = self.cap_style {s.to_native()} else {0},
      join_style: if let Some(s) = self.join_style {s.to_native()} else {0},
      fill_style: if let Some(s) = self.fill_style {s.to_native()} else {0},
      fill_rule: if let Some(r) = self.fill_rule {r.to_native()} else {0},
      arc_mode: if let Some(m) = self.arc_mode {m.to_native()} else {0},
      tile: if let Some(p) = self.tile {p as c_ulong} else {0},
      stipple: if let Some(p) = self.stipple {p as c_ulong} else {0},
      ts_x_origin: if let Some(n) = self.ts_x_origin {n as c_int} else {0},
      ts_y_origin: if let Some(n) = self.ts_y_origin {n as c_int} else {0},
      font: if let Some(f) = self.font {f as c_ulong} else {0},
      subwindow_mode: if let Some(m) = self.subwindow_mode {m.to_native()} else {0},
      graphics_exposures: if let Some(b) = self.graphics_exposures {if b {1} else {0}} else {0},
      clip_x_origin: if let Some(n) = self.clip_x_origin {n as c_int} else {0},
      clip_y_origin: if let Some(n) = self.clip_y_origin {n as c_int} else {0},
      clip_mask: if let Some(p) = self.clip_mask {p as c_ulong} else {0},
      dash_offset: if let Some(n) = self.dash_offset {n as c_int} else {0},
      dashes: 0, //TODO
    }
  }
}


//
// JoinStyle
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum JoinStyle {
  Miter,
  Round,
  Bevel,
}

impl ToNative<c_int> for JoinStyle {
  fn to_native (&self) -> c_int {
    match *self {
      JoinStyle::Miter => 0,
      JoinStyle::Round => 1,
      JoinStyle::Bevel => 2,
    }
  }
}


//
// LineStyle
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum LineStyle {
  Solid,
  OnOffDash,
  DoubleDash,
}

impl ToNative<c_int> for LineStyle {
  fn to_native (&self) -> c_int {
    match *self {
      LineStyle::Solid => 0,
      LineStyle::OnOffDash => 1,
      LineStyle::DoubleDash => 2,
    }
  }
}


//
// Rectangle
//


#[derive(Clone, Copy)]
pub struct Rectangle {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl ToNative<::ffi::XRectangle> for Rectangle {
  fn to_native (&self) -> ::ffi::XRectangle {
    ::ffi::XRectangle {
      x: self.x as c_short,
      y: self.y as c_short,
      width: self.width as c_ushort,
      height: self.height as c_ushort,
    }
  }
}


//
// SubwindowMode
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum SubwindowMode {
  ClipByChildren,
  IncludeInferiors,
}

impl ToNative<c_int> for SubwindowMode {
  fn to_native (&self) -> c_int {
    match *self {
      SubwindowMode::ClipByChildren => 0,
      SubwindowMode::IncludeInferiors => 1,
    }
  }
}
