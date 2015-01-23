// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

#![crate_name="xlib"]
#![crate_type="lib"]

#![allow(unstable)] // remove this when Rust is in beta

extern crate libc;

pub mod colormap;
pub mod cursor;
pub mod display;
pub mod drawable;
pub mod event;
pub mod pixmap;
pub mod screen;
pub mod visual;
pub mod window;

#[allow(dead_code, non_upper_case_globals)]
mod ffi;
mod internal;
#[cfg(test)]
mod test;
