// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use std::ffi::CString;
use std::mem::zeroed;
use std::ptr::{
  null,
  null_mut,
};
use std::sync::atomic::{
  AtomicBool,
  Ordering,
  ATOMIC_BOOL_INIT,
};

use libc::{
  c_char,
  c_int,
  c_uint,
  c_ulong,
  c_void,
};

use ::colormap::{
  Color,
  Colormap,
};
use ::drawable::{
  Drawable,
  Geometry,
};
use ::event::{
  Event,
  EventMask,
};
use ::gc::{
  ClipOrdering,
  Gcid,
  GcValues,
  Rectangle,
};
use ::internal::{
  FieldMask,
  FromNative,
  ToNative,
};
use ::visual::{
  Visual,
  VisualInfo,
  VisualTemplate,
};
use ::window::{
  SetWindowAttributes,
  SizeHints,
  Window,
  WindowAttributes,
  WindowClass,
};

// resource identifier types
pub type Atom = Xid;
pub type Xid = u32;


//
// Display
//


pub struct Display {
  ptr: *mut ::ffi::Display,
}

impl Display {
  pub fn alloc_color (&mut self, colormap: Colormap, color: Color) -> Option<Color> {
    unsafe {
      let mut xcolor = color.to_native();
      if ::ffi::XAllocColor(self.ptr, colormap as c_ulong, &mut xcolor) == 0 {
        return None;
      }
      return Some(FromNative::from_native(xcolor));
    }
  }

  pub fn black_pixel (&mut self, screen_num: i32) -> u32 {
    unsafe {
      if screen_num < 0 || screen_num >= self.screen_count() {
        return 0;
      }
      return ::ffi::XBlackPixel(self.ptr, screen_num as c_int) as u32;
    }
  }

  pub fn create_colormap (&mut self, window: Window, visual: Visual, alloc: bool) -> Colormap {
    unsafe {
      return ::ffi::XCreateColormap(self.ptr, window as c_ulong, visual.to_native(),
          if alloc {::ffi::AllocAll} else {::ffi::AllocNone}) as Colormap;
    }
  }

  pub fn create_gc (&mut self, drawable: Drawable, values: GcValues) -> Gcid {
    unsafe {
      let xgcvalues = values.to_native();
      return ::ffi::XCreateGC(self.ptr, drawable as c_ulong, values.field_mask(), &xgcvalues) as Gcid;
    }
  }

  pub fn create_simple_window (&mut self, parent: Window, x: i32, y: i32, width: i32, height: i32, border_width: i32,
      border_pixel: u32, background_pixel: u32) -> Window
  {
    unsafe {
      return ::ffi::XCreateSimpleWindow(self.ptr, parent as c_ulong, x as c_int, y as c_int, width as c_uint,
          height as c_uint, border_width as c_uint, border_pixel as c_ulong, background_pixel as c_ulong) as Window;
    }
  }

  pub fn create_window (&mut self, parent: Window, x: i32, y: i32, width: i32, height: i32, border_width: i32,
      depth: Option<i32>, class: WindowClass, visual: Option<Visual>, attr: SetWindowAttributes) -> Window
  {
    unsafe {
      let c_depth = if let Some(d) = depth {d as c_int} else {0};
      let c_visual = if let Some(v) = visual {v.to_native()} else {null()};
      let c_attr = attr.to_native();
      return ::ffi::XCreateWindow(self.ptr, parent as c_ulong, x as c_int, y as c_int, width as c_uint,
          height as c_uint, border_width as c_uint, c_depth, class.to_native(), c_visual, attr.field_mask(), &c_attr)
          as Window;
    }
  }

  pub fn default_colormap (&mut self, screen_num: i32) -> Colormap {
    unsafe {
      if screen_num < 0 || screen_num >= self.screen_count() {
        return 0;
      }
      return ::ffi::XDefaultColormap(self.ptr, screen_num as c_int) as Colormap;
    }
  }

  pub fn default_screen (&mut self) -> i32 {
    unsafe {
      return ::ffi::XDefaultScreen(self.ptr) as i32;
    }
  }

  pub fn default_visual (&mut self, screen_num: i32) -> Visual {
    unsafe {
      if screen_num < 0 || screen_num >= self.screen_count() {
        return FromNative::from_native(null());
      }
      return FromNative::from_native(::ffi::XDefaultVisual(self.ptr, screen_num as c_int));
    }
  }

  pub fn destroy_window (&mut self, window: Window) {
    unsafe {
      return ::ffi::XDestroyWindow(self.ptr, window as c_ulong);;
    }
  }

  pub fn draw_line (&mut self, drawable: Drawable, gc: Gcid, x0: i32, y0: i32, x1: i32, y1: i32) {
    unsafe {
      ::ffi::XDrawLine(self.ptr, drawable as c_ulong, gc as c_ulong, x0 as c_int, y0 as c_int,
          x1 as c_int, y1 as c_int);
    }
  }

  pub fn draw_rectangle (&mut self, drawable: Drawable, gc: Gcid, x: i32, y: i32, width: i32, height: i32) {
    unsafe {
      ::ffi::XDrawRectangle(self.ptr, drawable as c_ulong, gc as c_ulong, x as c_int, y as c_int,
          width as c_uint, height as c_uint);
    }
  }

  pub fn fetch_name (&mut self, window: Window) -> String {
    unsafe {
      let mut name_ptr: *mut c_char = null_mut();
      if ::ffi::XFetchName(self.ptr, window as c_ulong, &mut name_ptr) == 0 {
        if name_ptr != null_mut() {
          ::ffi::XFree(name_ptr as *mut c_void);
        }
        return String::new();
      }
      if name_ptr == null_mut() {
        return String::new();
      }
      let name_byte_ptr = name_ptr as *const c_char as *const u8;
      let name_len = ::libc::strlen(name_ptr as *const c_char) as usize;
      let name_slice = ::std::slice::from_raw_buf(&name_byte_ptr, name_len);
      let name_string = String::from_utf8_lossy(name_slice).into_owned();
      ::ffi::XFree(name_ptr as *mut c_void);
      return name_string;
    }
  }

  pub fn fill_rectangle (&mut self, drawable: Drawable, gc: Gcid, x: i32, y: i32, width: i32, height: i32) {
    unsafe {
      ::ffi::XFillRectangle(self.ptr, drawable as c_ulong, gc as c_ulong, x as c_int, y as c_int,
          width as c_uint, height as c_uint);
    }
  }

  pub fn flush (&mut self) {
    unsafe {
      ::ffi::XFlush(self.ptr);
    }
  }

  pub fn free_colormap (&mut self, colormap: Colormap) {
    unsafe {
      ::ffi::XFreeColormap(self.ptr, colormap as c_ulong);
    }
  }

  pub fn free_gc (&mut self, gc: Gcid) {
    unsafe {
      ::ffi::XFreeGC(self.ptr, gc as c_ulong);
    }
  }

  pub fn get_geometry (&mut self, drawable: Drawable) -> Option<Geometry> {
    unsafe {
      let mut root = 0;
      let mut x = 0;
      let mut y = 0;
      let mut width = 0;
      let mut height = 0;
      let mut border_width = 0;
      let mut depth = 0;

      if ::ffi::XGetGeometry(self.ptr, drawable as c_ulong, &mut root, &mut x, &mut y, &mut width, &mut height,
          &mut border_width, &mut depth) == 0
      {
        return None;
      }

      let geometry = Geometry {
        root: root as Window,
        x: x as i32,
        y: y as i32,
        width: width as i32,
        height: height as i32,
        border_width: border_width as i32,
        depth: depth as i32,
      };
      return Some(geometry);
    }
  }

  pub fn get_visual_info (&mut self, template: VisualTemplate) -> Vec<VisualInfo> {
    unsafe {
      let mut info_vec = Vec::new();
      let xtemplate = template.to_native();
      let mut nitems = 0;
      let xinfo_ptr = ::ffi::XGetVisualInfo(self.ptr, template.field_mask(), &xtemplate, &mut nitems);
      if xinfo_ptr == null_mut() {
        return info_vec;
      }
      let xinfo_const_ptr = xinfo_ptr as *const ::ffi::XVisualInfo;
      let xinfo_slice = ::std::slice::from_raw_buf(&xinfo_const_ptr, nitems as usize);
      for xinfo in xinfo_slice.iter() {
        if let Some(info) = FromNative::from_native(*xinfo) {
          info_vec.push(info);
        } else {
          error!("XGetVisualInfo returned invalid data");
        }
      }
      ::ffi::XFree(xinfo_ptr as *mut c_void);
      return info_vec;
    }
  }

  pub fn get_window_attributes (&mut self, window: Window) -> Option<WindowAttributes> {
    unsafe {
      let mut xattr: ::ffi::XWindowAttributes = zeroed();
      if ::ffi::XGetWindowAttributes(self.ptr, window as c_ulong, &mut xattr) == 0 {
        return None;
      }
      if let Some(attr) = FromNative::from_native(xattr) {
        return Some(attr);
      } else {
        error!("XGetWindowAttributes returned invalid data");
        return None;
      }
    }
  }

  pub fn intern_atom (&mut self, name: &str, only_if_exists: bool) -> Option<Atom> {
    unsafe {
      let name_c_str = CString::from_slice(name.as_bytes());
      let atom = ::ffi::XInternAtom(self.ptr, name_c_str.as_ptr(), if only_if_exists {1} else {0});
      if atom == 0 {
        return None;
      } else {
        return Some(atom as Atom);
      }
    }
  }

  pub fn map_window (&mut self, window: Window) {
    unsafe {
      ::ffi::XMapWindow(self.ptr, window as c_ulong);
    }
  }

  pub fn move_window (&mut self, window: Window, x: i32, y: i32) {
    unsafe {
      ::ffi::XMoveWindow(self.ptr, window as c_ulong, x as c_int, y as c_int);
    }
  }

  pub fn next_event (&mut self) -> Event {
    unsafe {
      let mut xevent: ::ffi::XEvent = zeroed();
      loop {
        ::ffi::XNextEvent(self.ptr, &mut xevent);
        if let Some(event) = FromNative::from_native(xevent) {
          return event;
        }
      }
    }
  }

  pub fn open (name: &str) -> Option<Display> {
    unsafe {
      init();
      let name_c_str = CString::from_slice(name.as_bytes());
      let ptr = ::ffi::XOpenDisplay(name_c_str.as_ptr());
      if ptr == null_mut() {
        return None;
      }
      let display = Display {
        ptr: ptr,
      };
      return Some(display);
    }
  }

  pub fn open_default () -> Option<Display> {
    unsafe {
      init();
      let ptr = ::ffi::XOpenDisplay(null());
      if ptr == null_mut() {
        return None;
      }
      let display = Display {
        ptr: ptr,
      };
      return Some(display);
    }
  }

  pub fn pending (&self) -> i32 {
    unsafe {
      ::ffi::XPending(self.ptr) as i32
    }
  }

  pub fn resize_window (&mut self, window: Window, width: i32, height: i32) {
    unsafe {
      ::ffi::XResizeWindow(self.ptr, window as c_ulong, width as c_uint, height as c_uint);
    }
  }

  pub fn root_window (&mut self, screen_num: i32) -> Window {
    unsafe {
      if screen_num < 0 || screen_num >= self.screen_count() {
        return 0;
      }
      return ::ffi::XRootWindow(self.ptr, screen_num as c_int) as Window;
    }
  }

  pub fn screen_count (&mut self) -> i32 {
    unsafe {
      return ::ffi::XScreenCount(self.ptr);
    }
  }

  pub fn send_event (&mut self, propagate: bool, event_mask: EventMask, event: Event) -> bool {
    unsafe {
      let xevent = event.to_native();
      let window = (*(&xevent as *const ::ffi::XEvent as *const ::ffi::XAnyEvent)).window;
      return ::ffi::XSendEvent(self.ptr, window, if propagate {1} else {0}, event_mask.to_native(), &xevent) != 0;
    }
  }

  pub fn set_clip_rectangles (&mut self, gc: Gcid, x_origin: i32, y_origin: i32, rects: &[Rectangle],
      ordering: ClipOrdering)
  {
    unsafe {
      let mut xrects: Vec<::ffi::XRectangle> = Vec::with_capacity(rects.len());
      for rect in rects.iter() {
        xrects.push(rect.to_native());
      }
      ::ffi::XSetClipRectangles(self.ptr, gc as c_ulong, x_origin as c_int, y_origin as c_int, &xrects[0],
          xrects.len() as c_int, ordering.to_native());
    }
  }

  pub fn set_foreground (&mut self, gc: Gcid, pixel: u32) {
    unsafe {
      ::ffi::XSetForeground(self.ptr, gc as c_ulong, pixel as c_ulong);
    }
  }

  pub fn set_wm_normal_hints (&mut self, window: Window, hints: SizeHints) {
    unsafe {
      let xhints = hints.to_native();
      ::ffi::XSetWMNormalHints(self.ptr, window as c_ulong, &xhints);
    }
  }

  pub fn set_wm_protocols (&mut self, window: Window, protocols: &[Atom]) {
    unsafe {
      let mut protocol_vec: Vec<c_ulong> = Vec::with_capacity(protocols.len());
      for protocol in protocols.iter() {
        protocol_vec.push(*protocol as c_ulong);
      }
      if ::ffi::XSetWMProtocols(self.ptr, window as c_ulong, &protocol_vec[0], protocols.len() as c_int) == 0 {
        error!("XSetWMProtocols failed");
      }
    }
  }

  pub fn store_name (&mut self, window: Window, name: &str) {
    unsafe {
      let name_c_str = CString::from_slice(name.as_bytes());
      ::ffi::XStoreName(self.ptr, window as c_ulong, name_c_str.as_ptr());
    }
  }

  pub fn unmap_window (&mut self, window: Window) {
    unsafe {
      ::ffi::XUnmapWindow(self.ptr, window as c_ulong);
    }
  }

  pub fn white_pixel (&mut self, screen_num: i32) -> u32 {
    unsafe {
      if screen_num < 0 || screen_num >= self.screen_count() {
        return 0;
      }
      return ::ffi::XWhitePixel(self.ptr, screen_num as c_int) as u32;
    }
  }
}

impl Drop for Display {
  fn drop (&mut self) {
    unsafe {
      ::ffi::XCloseDisplay(self.ptr);
    }
  }
}


//
// initialize before connecting
//


static mut _was_init: AtomicBool = ATOMIC_BOOL_INIT;

unsafe extern "C"
fn handle_error (display: *mut ::ffi::Display, event: *const ::ffi::XErrorEvent) -> c_int {
  let mut error_buf = [0u8; 64];
  let error_ptr = &mut error_buf[0] as *mut u8 as *mut c_char;
  ::ffi::XGetErrorText(display, (*event).error_code as c_int, error_ptr, error_buf.len() as c_int);
  let error_string = String::from_utf8_lossy(&error_buf[]).into_owned();
  error!("Xlib: Serial {}, Error Code {} ({}), Request Code {}, Minor Code {}, Resource ID {}",
      (*event).serial, (*event).error_code, error_string, (*event).request_code, (*event).minor_code,
      (*event).resourceid);
  return 0;
}

#[allow(unused_variables)]
unsafe extern "C"
fn handle_io_error (display: *mut ::ffi::Display) -> c_int {
  panic!("Xlib: A fatal I/O error has occurred!");
}

fn init () {
  unsafe {
    if !_was_init.swap(false, Ordering::SeqCst) {
      ::ffi::XSetErrorHandler(handle_error);
      ::ffi::XSetIOErrorHandler(handle_io_error);
    }
  }
}
