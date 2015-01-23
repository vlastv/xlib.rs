// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use libc::{
  c_int,
  c_long,
  c_uint,
  c_ulong,
};

use ::colormap::Colormap;
use ::cursor::Cursor;
use ::display::Xid;
use ::event::EventMask;
use ::internal::{
  FieldMask,
  FromNative,
  ToNative,
};
use ::pixmap::Pixmap;
use ::screen::Screen;
use ::visual::Visual;

/** Window identifier type. */
pub type Window = Xid;


//
// AspectRatioHint
//


#[derive(Clone, Copy)]
pub struct AspectRatioHint {
  pub min_numerator: i32,
  pub min_denominator: i32,
  pub max_numerator: i32,
  pub max_denominator: i32,
}


//
// BackingStore
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum BackingStore {
  NotUseful,
  WhenMapped,
  Always,
}

impl FromNative<c_int> for Option<BackingStore> {
  fn from_native (num: c_int) -> Option<BackingStore> {
    match num {
      0 => Some(BackingStore::NotUseful),
      1 => Some(BackingStore::WhenMapped),
      2 => Some(BackingStore::Always),
      _ => None,
    }
  }
}

impl ToNative<c_int> for BackingStore {
  fn to_native (&self) -> c_int {
    match *self {
      BackingStore::NotUseful => 0,
      BackingStore::WhenMapped => 1,
      BackingStore::Always => 2,
    }
  }
}


//
// Gravity
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Gravity {
  None,
  NorthWest,
  North,
  NorthEast,
  West,
  Center,
  East,
  SouthWest,
  South,
  SouthEast,
  Static,
}

impl FromNative<c_int> for Option<Gravity> {
  fn from_native (num: c_int) -> Option<Gravity> {
    match num {
      0 => Some(Gravity::None),
      1 => Some(Gravity::NorthWest),
      2 => Some(Gravity::North),
      3 => Some(Gravity::NorthEast),
      4 => Some(Gravity::West),
      5 => Some(Gravity::Center),
      6 => Some(Gravity::East),
      7 => Some(Gravity::SouthWest),
      8 => Some(Gravity::South),
      9 => Some(Gravity::SouthEast),
      10 => Some(Gravity::Static),
      _ => None,
    }
  }
}

impl ToNative<c_int> for Gravity {
  fn to_native (&self) -> c_int {
    match *self {
      Gravity::None => 0,
      Gravity::NorthWest => 1,
      Gravity::North => 2,
      Gravity::NorthEast => 3,
      Gravity::West => 4,
      Gravity::Center => 5,
      Gravity::East => 6,
      Gravity::SouthWest => 7,
      Gravity::South => 8,
      Gravity::SouthEast => 9,
      Gravity::Static => 10,
    }
  }
}


//
// MapState
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MapState {
  Unmapped,
  Unviewable,
  Viewable,
}

impl FromNative<c_int> for Option<MapState> {
  fn from_native (num: c_int) -> Option<MapState> {
    match num {
      0 => Some(MapState::Unmapped),
      1 => Some(MapState::Unviewable),
      2 => Some(MapState::Viewable),
      _ => None,
    }
  }
}


//
// SetWindowAttributes
//


#[derive(Clone, Copy, Default)]
pub struct SetWindowAttributes {
  pub background_pixmap: Option<Pixmap>,
  pub background_pixel: Option<u32>,
  pub border_pixmap: Option<Pixmap>,
  pub border_pixel: Option<u32>,
  pub bit_gravity: Option<Gravity>,
  pub win_gravity: Option<Gravity>,
  pub backing_store: Option<BackingStore>,
  pub backing_planes: Option<u32>,
  pub backing_pixel: Option<u32>,
  pub save_under: Option<bool>,
  pub event_mask: Option<EventMask>,
  pub do_not_propagate_mask: Option<EventMask>,
  pub override_redirect: Option<bool>,
  pub colormap: Option<Colormap>,
  pub cursor: Option<Cursor>,
}

impl FieldMask<c_ulong> for SetWindowAttributes {
  fn field_mask (&self) -> c_ulong {
    let mut mask: c_ulong = 0;
    if let Some(_) = self.background_pixmap { mask |= 0x0001; }
    if let Some(_) = self.background_pixel { mask |= 0x0002; }
    if let Some(_) = self.border_pixmap { mask |= 0x0004; }
    if let Some(_) = self.border_pixel { mask |= 0x0008; }
    if let Some(_) = self.bit_gravity { mask |= 0x0010; }
    if let Some(_) = self.win_gravity { mask |= 0x0020; }
    if let Some(_) = self.backing_store { mask |= 0x0040; }
    if let Some(_) = self.backing_planes { mask |= 0x0080; }
    if let Some(_) = self.backing_pixel { mask |= 0x0100; }
    if let Some(_) = self.override_redirect { mask |= 0x0200; }
    if let Some(_) = self.save_under { mask |= 0x0400; }
    if let Some(_) = self.event_mask { mask |= 0x0800; }
    if let Some(_) = self.do_not_propagate_mask { mask |= 0x1000; }
    if let Some(_) = self.colormap { mask |= 0x2000; }
    if let Some(_) = self.cursor { mask |= 0x4000; }
    return mask;
  }
}

impl ToNative<::ffi::XSetWindowAttributes> for SetWindowAttributes {
  fn to_native (&self) -> ::ffi::XSetWindowAttributes {
    ::ffi::XSetWindowAttributes {
      background_pixmap: if let Some(p) = self.background_pixmap {p as c_ulong} else {0},
      background_pixel: if let Some(p) = self.background_pixel {p as c_ulong} else {0},
      border_pixmap: if let Some(p) = self.border_pixmap {p as c_ulong} else {0},
      border_pixel: if let Some(p) = self.border_pixel {p as c_ulong} else {0},
      bit_gravity: if let Some(g) = self.bit_gravity {g.to_native()} else {0},
      win_gravity: if let Some(g) = self.win_gravity {g.to_native()} else {0},
      backing_store: if let Some(bs) = self.backing_store {bs.to_native()} else {0},
      backing_planes: if let Some(p) = self.backing_planes {p as c_ulong} else {0},
      backing_pixel: if let Some(p) = self.backing_pixel {p as c_ulong} else {0},
      save_under: if let Some(b) = self.save_under {if b {1} else {0}} else {0},
      event_mask: if let Some(em) = self.event_mask {em.to_native()} else {0},
      do_not_propagate_mask: if let Some(em) = self.do_not_propagate_mask {em.to_native()} else {0},
      override_redirect: if let Some(b) = self.override_redirect {if b {1} else {0}} else {0},
      colormap: if let Some(c) = self.colormap {c as c_ulong} else {0},
      cursor: if let Some(c) = self.cursor {c as c_ulong} else {0},
    }
  }
}


//
// SizeHints
//


#[derive(Clone, Copy, Default)]
pub struct SizeHints {
  pub position: Option<(i32, i32)>,
  pub size: Option<(i32, i32)>,
  pub min_size: Option<(i32, i32)>,
  pub max_size: Option<(i32, i32)>,
  pub resize_inc: Option<(i32, i32)>,
  pub aspect: Option<AspectRatioHint>,
  pub base_size: Option<(i32, i32)>,
  pub win_gravity: Option<Gravity>,
}

impl FieldMask<c_long> for SizeHints {
  fn field_mask (&self) -> c_long {
    let mut mask: c_long = 0;
    if let Some(_) = self.position { mask |= 0x0004; }
    if let Some(_) = self.size { mask |= 0x0008; }
    if let Some(_) = self.min_size { mask |= 0x0010; }
    if let Some(_) = self.max_size { mask |= 0x0020; }
    if let Some(_) = self.resize_inc { mask |= 0x0040; }
    if let Some(_) = self.aspect { mask |= 0x0080; }
    if let Some(_) = self.base_size { mask |= 0x0100; }
    if let Some(_) = self.win_gravity { mask |= 0x0200; }
    return mask;
  }
}

impl ToNative<::ffi::XSizeHints> for SizeHints {
  fn to_native (&self) -> ::ffi::XSizeHints {
    ::ffi::XSizeHints {
      flags: self.field_mask(),
      x: if let Some((x, _)) = self.position {x as c_int} else {0},
      y: if let Some((_, y)) = self.position {y as c_int} else {0},
      width: if let Some((w, _)) = self.size {w as c_int} else {0},
      height: if let Some((_, h)) = self.size {h as c_int} else {0},
      min_width: if let Some((w, _)) = self.min_size {w as c_int} else {0},
      min_height: if let Some((_, h)) = self.min_size {h as c_int} else {0},
      max_width: if let Some((w, _)) = self.max_size {w as c_int} else {0},
      max_height: if let Some((_, h)) = self.max_size {h as c_int} else {0},
      width_inc: if let Some((w, _)) = self.resize_inc {w as c_int} else {0},
      height_inc: if let Some((_, h)) = self.resize_inc {h as c_int} else {0},
      min_aspect: if let Some(aspect) = self.aspect {::ffi::Point{x: aspect.min_numerator as c_int,
          y: aspect.min_denominator as c_int}} else {::ffi::Point{x: 0, y: 0}},
      max_aspect: if let Some(aspect) = self.aspect {::ffi::Point{x: aspect.max_numerator as c_int,
          y: aspect.max_denominator as c_int}} else {::ffi::Point{x: 0, y: 0}},
      base_width: if let Some((w, _)) = self.base_size {w as c_int} else {0},
      base_height: if let Some((_, h)) = self.base_size {h as c_int} else {0},
      win_gravity: if let Some(g) = self.win_gravity {g.to_native()} else {0},
    }
  }
}


//
// WindowAttributes
//


#[derive(Clone, Copy)]
pub struct WindowAttributes {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub border_width: i32,
  pub depth: i32,
  pub visual: Visual,
  pub root: Window,
  pub class: WindowClass,
  pub bit_gravity: Gravity,
  pub win_gravity: Gravity,
  pub backing_store: BackingStore,
  pub backing_planes: u32,
  pub backing_pixel: u32,
  pub save_under: bool,
  pub colormap: Colormap, // TODO: check to see if we can wrap this in Option<> and remove map_installed
  pub map_installed: bool,
  pub map_state: MapState,
  pub all_event_masks: EventMask,
  pub your_event_mask: EventMask,
  pub do_not_propagate_mask: EventMask,
  pub override_redirect: bool,
  pub screen: Screen,
}

impl FromNative<::ffi::XWindowAttributes> for Option<WindowAttributes> {
  fn from_native (xattr: ::ffi::XWindowAttributes) -> Option<WindowAttributes> {
    let attr = WindowAttributes {
      x: xattr.x as i32,
      y: xattr.y as i32,
      width: xattr.width as i32,
      height: xattr.height as i32,
      border_width: xattr.border_width as i32,
      depth: xattr.depth as i32,
      visual: FromNative::from_native(xattr.visual),
      root: xattr.root as Window,
      class: if let Some(c) = FromNative::from_native(xattr.class) {c} else { return None; },
      bit_gravity: if let Some(g) = FromNative::from_native(xattr.bit_gravity) {g} else { return None; },
      win_gravity: if let Some(g) = FromNative::from_native(xattr.win_gravity) {g} else { return None; },
      backing_store: if let Some(bs) = FromNative::from_native(xattr.backing_store) {bs} else { return None; },
      backing_planes: xattr.backing_planes as u32,
      backing_pixel: xattr.backing_pixel as u32,
      save_under: xattr.save_under != 0,
      colormap: xattr.colormap as Colormap,
      map_installed: xattr.map_installed != 0,
      map_state: if let Some(ms) = FromNative::from_native(xattr.map_state) {ms} else { return None; },
      all_event_masks: FromNative::from_native(xattr.all_event_masks),
      your_event_mask: FromNative::from_native(xattr.your_event_mask),
      do_not_propagate_mask: FromNative::from_native(xattr.do_not_propagate_mask),
      override_redirect: xattr.override_redirect != 0,
      screen: FromNative::from_native(xattr.screen),
    };
    return Some(attr);
  }
}


//
// WindowClass
//


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum WindowClass {
  InputOutput,
  InputOnly,
}

impl FromNative<c_int> for Option<WindowClass> {
  fn from_native (num: c_int) -> Option<WindowClass> {
    match num {
      1 => Some(WindowClass::InputOutput),
      2 => Some(WindowClass::InputOnly),
      _ => None,
    }
  }
}

impl ToNative<c_uint> for WindowClass {
  fn to_native (&self) -> c_uint {
    match *self {
      WindowClass::InputOutput => 1,
      WindowClass::InputOnly => 2,
    }
  }
}


//
// public functions
//


pub fn all_planes () -> u32 {
  unsafe {
    return ::ffi::XAllPlanes() as u32;
  }
}
