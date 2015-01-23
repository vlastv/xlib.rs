// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use ::display::Xid;
use ::window::Window;

/** Drawable resource identifier type. */
pub type Drawable = Xid;


//
// Geometry
//


#[derive(Clone, Copy)]
pub struct Geometry {
  pub root: Window,
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub border_width: i32,
  pub depth: i32,
}
