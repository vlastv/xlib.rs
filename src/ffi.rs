// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use std;
use std::default::Default;
use libc::{
  c_char,
  c_int,
  c_long,
  c_short,
  c_uint,
  c_ulong,
  c_ushort,
  c_void,
};


//
// functions
//


#[link(name="X11")]
extern "C" {
  pub fn XAllocColor (display: *mut Display, colormap: Colormap, color: *mut XColor) -> Status;
  pub fn XAllPlanes () -> c_ulong;
  pub fn XBlackPixel (display: *mut Display, screen_num: c_int) -> c_ulong;
  pub fn XCloseDisplay (display: *mut Display);
  pub fn XCreateColormap (display: *mut Display, window: Window, visual: *const Visual, alloc: c_int) -> Colormap;
  pub fn XCreateGC (display: *mut Display, drawable: Drawable, valuemask: c_ulong, values: *const XGCValues) -> GC;
  pub fn XCreateSimpleWindow (display: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint, height: c_uint,
      border_width: c_uint, border_pixel: c_ulong, background_pixel: c_ulong) -> Window;
  pub fn XCreateWindow (display: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint, height: c_uint,
      border_width: c_uint, depth: c_int, class: c_uint, visual: *const Visual, valuemask: c_ulong,
      attributes: *const XSetWindowAttributes) -> Window;
  pub fn XDefaultColormap (display: *mut Display, screen_num: c_int) -> Colormap;
  pub fn XDefaultScreen (display: *mut Display) -> c_int;
  pub fn XDefaultVisual (display: *mut Display, screen_num: c_int) -> *const Visual;
  pub fn XDestroyWindow (display: *mut Display, window: Window);
  pub fn XDrawLine (display: *mut Display, drawable: Drawable, gc: GC, x0: c_int, y0: c_int, x1: c_int, y1: c_int);
  pub fn XDrawRectangle (display: *mut Display, drawable: Drawable, gc: GC, x: c_int, y: c_int, width: c_uint,
      height: c_uint);
  pub fn XFetchName (display: *mut Display, window: Window, name: *mut *mut c_char) -> Status;
  pub fn XFillRectangle (display: *mut Display, drawable: Drawable, gc: GC, x: c_int, y: c_int, width: c_uint,
      height: c_uint);
  pub fn XFlush (display: *mut Display);
  pub fn XFree (mem: *mut c_void);
  pub fn XFreeColormap (display: *mut Display, colormap: Colormap);
  pub fn XFreeGC (display: *mut Display, gc: GC);
  pub fn XGetGeometry (display: *mut Display, drawable: Drawable, root_return: *mut Window, x_result: *mut c_int,
      y_return: *mut c_int, width_return: *mut c_uint, height_return: *mut c_uint, border_width_return: *mut c_uint,
      depth_return: *mut c_uint) -> Status;
  pub fn XGetVisualInfo (display: *mut Display, vinfo_mask: c_long, vinfo_template: *const XVisualInfo,
      count: *mut c_int) -> *mut XVisualInfo;
  pub fn XGetWindowAttributes (display: *mut Display, window: Window, attr: *mut XWindowAttributes) -> Status;
  pub fn XInternAtom (display: *mut Display, name: *const c_char, only_if_exists: Bool) -> Atom;
  pub fn XMapWindow (display: *mut Display, window: Window);
  pub fn XMoveWindow (display: *mut Display, window: Window, x: c_int, y: c_int);
  pub fn XNextEvent (display: *mut Display, event: *mut XEvent);
  pub fn XOpenDisplay (name: *const c_char) -> *mut Display;
  pub fn XPending (display: *mut Display) -> c_int;
  pub fn XResizeWindow (display: *mut Display, window: Window, width: c_uint, height: c_uint);
  pub fn XRootWindow (display: *mut Display, screen_num: c_int) -> Window;
  pub fn XScreenCount (display: *mut Display) -> c_int;
  pub fn XSendEvent (display: *mut Display, window: Window, propagate: Bool, event_mask: c_long, event: *const XEvent)
      -> Bool;
  pub fn XSetClipRectangles (display: *mut Display, gc: GC, clip_x_origin: c_int, clip_y_origin: c_int,
      rectangles: *const XRectangle, num_rectangles: c_int, ordering: c_int);
  pub fn XSetForeground (display: *mut Display, gc: GC, pixel: c_ulong);
  pub fn XSetWMNormalHints (display: *mut Display, window: Window, hints: *const XSizeHints);
  pub fn XSetWMProtocols (display: *mut Display, window: Window, protocols: *const Atom, count: c_int) -> Status;
  pub fn XStoreName (display: *mut Display, window: Window, name: *const c_char);
  pub fn XUnmapWindow (display: *mut Display, window: Window);
  pub fn XVisualIDFromVisual (visual: *const Visual) -> VisualID;
  pub fn XWhitePixel (display: *mut Display, screen_num: c_int) -> c_ulong;
}


//
// types
//


// misc types
pub type Atom = XID;
pub type Bool = c_int;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type GC = XID;
pub type GLXDrawable = XID;
pub type Pixmap = XID;
pub type Status = Bool;
pub type VisualID = XID;
pub type Window = XID;
pub type XID = c_ulong;

// opaque structs
#[repr(C)] pub struct Display;
#[repr(C)] pub struct GLXContext_Rec;
#[repr(C)] pub struct GLXFBConfig_Rec;
#[repr(C)] pub struct Screen;
#[repr(C)] pub struct Visual;

pub type GLXContext = *mut GLXContext_Rec;
pub type GLXFBConfig = *mut GLXFBConfig_Rec;

// Point
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Point {
  pub x: c_int,
  pub y: c_int,
}

// XAnyEvent
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XAnyEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
}

// XClientMessageEvent
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XClientMessageEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub message_type: Atom,
  pub format: c_int,
  data: [c_long; 5],
}

impl XClientMessageEvent {
  pub fn get_byte (&self, index: usize) -> c_char {
    unsafe {
      let ptr = &self.data[0] as *const c_long as *const c_char;
      return std::slice::from_raw_buf(&ptr, 20)[index];
    }
  }

  pub fn get_long (&self, index: usize) -> c_long {
    self.data[index]
  }

  pub fn get_short (&self, index: usize) -> c_short {
    unsafe {
      let ptr = &self.data[0] as *const c_long as *const c_short;
      return std::slice::from_raw_buf(&ptr, 10)[index];
    }
  }

  pub fn set_byte (&mut self, index: usize, value: c_char) {
    unsafe {
      let ptr = &mut self.data[0] as *mut c_long as *mut c_char;
      std::slice::from_raw_mut_buf(&ptr, 20)[index] = value;
    }
  }

  pub fn set_long (&mut self, index: usize, value: c_long) {
    self.data[index] = value;
  }

  pub fn set_short (&mut self, index: usize, value: c_short) {
    unsafe {
      let ptr = &mut self.data[0] as *mut c_long as *mut c_short;
      std::slice::from_raw_mut_buf(&ptr, 10)[index] = value;
    }
  }
}

// XColor
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct XColor {
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub flags: c_char,
  pub pad: c_char,
}

// XConfigureEvent
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XConfigureEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub above: Window,
  pub override_redirect: Bool,
}

// XDestroyWindowEvent
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XDestroyWindowEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
}

// XEvent
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XEvent {
  pad: [c_long; 24],
}

impl XEvent {
  pub fn kind (&self) -> c_int {
    unsafe {
      *(&self.pad[0] as *const c_long as *const c_int)
    }
  }
}

// XExposeEvent
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XExposeEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
}

// XGCValues
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct XGCValues {
  pub function: c_int,
  pub plane_mask: c_ulong,
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub line_width: c_int,
  pub line_style: c_int,
  pub cap_style: c_int,
  pub join_style: c_int,
  pub fill_style: c_int,
  pub fill_rule: c_int,
  pub arc_mode: c_int,
  pub tile: Pixmap,
  pub stipple: Pixmap,
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,
  pub font: Font,
  pub subwindow_mode: c_int,
  pub graphics_exposures: Bool,
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,
  pub clip_mask: Pixmap,
  pub dash_offset: c_int,
  pub dashes: c_char,
}

// XMapEvent
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XMapEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub override_redirect: Bool,
}

// XRectangle
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct XRectangle {
  pub x: c_short,
  pub y: c_short,
  pub width: c_ushort,
  pub height: c_ushort,
}

// XSetWindowAttributes
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct XSetWindowAttributes {
  pub background_pixmap: Pixmap,
  pub background_pixel: c_ulong,
  pub border_pixmap: Pixmap,
  pub border_pixel: c_ulong,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub colormap: Colormap,
  pub cursor: Cursor,
}

// XSizeHints
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct XSizeHints {
  pub flags: c_long,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub min_width: c_int,
  pub min_height: c_int,
  pub max_width: c_int,
  pub max_height: c_int,
  pub width_inc: c_int,
  pub height_inc: c_int,
  pub min_aspect: Point,
  pub max_aspect: Point,
  pub base_width: c_int,
  pub base_height: c_int,
  pub win_gravity: c_int,
}

// XUnmapEvent
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct XUnmapEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub from_configure: Bool,
}

// XVisualInfo
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct XVisualInfo {
  pub visual: *const Visual,
  pub visualid: VisualID,
  pub screen: c_int,
  pub depth: c_uint,
  pub class: c_int,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub colormap_size: c_int,
  pub bits_per_rgb: c_int,
}

// XWindowAttributes
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XWindowAttributes {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub depth: c_int,
  pub visual: *const Visual,
  pub root: Window,
  pub class: c_int,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub colormap: Colormap,
  pub map_installed: Bool,
  pub map_state: c_int,
  pub all_event_masks: c_long,
  pub your_event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub screen: *const Screen,
}


//
// constants
//


// clip rect ordering
pub const Unsorted: c_int = 0;
pub const YSorted: c_int = 1;
pub const YXSorted: c_int = 2;
pub const YXBanded: c_int = 3;

// colormap allocation
pub const AllocNone: c_int = 0;
pub const AllocAll: c_int = 0;

// event kind
pub const KeyPress: c_int = 2;
pub const KeyRelease: c_int = 3;
pub const ButtonPress: c_int = 4;
pub const ButtonRelease: c_int = 5;
pub const MotionNotify: c_int = 6;
pub const EnterNotify: c_int = 7;
pub const LeaveNotify: c_int = 8;
pub const FocusIn: c_int = 9;
pub const FocusOut: c_int = 10;
pub const KeymapNotify: c_int = 11;
pub const Expose: c_int = 12;
pub const GraphicsExpose: c_int = 13;
pub const NoExpose: c_int = 14;
pub const VisibilityNotify: c_int = 15;
pub const CreateNotify: c_int = 16;
pub const DestroyNotify: c_int = 17;
pub const UnmapNotify: c_int = 18;
pub const MapNotify: c_int = 19;
pub const MapRequest: c_int = 20;
pub const ReparentNotify: c_int = 21;
pub const ConfigureNotify: c_int = 22;
pub const ConfigureRequest: c_int = 23;
pub const GravityNotify: c_int = 24;
pub const ResizeRequest: c_int = 25;
pub const CirculateNotify: c_int = 26;
pub const CirculateRequest: c_int = 27;
pub const PropertyNotify: c_int = 28;
pub const SelectionClear: c_int = 29;
pub const SelectionRequest: c_int = 30;
pub const SelectionNotify: c_int = 31;
pub const ColormapNotify: c_int = 32;
pub const ClientMessage: c_int = 33;
pub const MappingNotify: c_int = 34;

// map static
pub const IsUnmapped: c_int = 0;
pub const IsUnviewable: c_int = 1;
pub const IsViewable: c_int = 2;

// size hints mask
pub const USPosition: c_long = 0x0001;
pub const USSize: c_long = 0x0002;
pub const PPosition: c_long = 0x0004;
pub const PSize: c_long = 0x0008;
pub const PMinSize: c_long = 0x0010;
pub const PMaxSize: c_long = 0x0020;
pub const PResizeInc: c_long = 0x0040;
pub const PAspect: c_long = 0x0080;
pub const PBaseSize: c_long = 0x0100;
pub const PWinGravity: c_long = 0x0200;
pub const PAllHints: c_long = PPosition | PSize | PMinSize | PMaxSize | PResizeInc | PAspect;

// visual class
pub const StaticGray: c_int = 0;
pub const GrayScale: c_int = 1;
pub const StaticColor: c_int = 2;
pub const PseudoColor: c_int = 3;
pub const TrueColor: c_int = 4;
pub const DirectColor: c_int = 5;

// visual info mask
pub const VisualNoMask: c_long = 0;
pub const VisualIDMask: c_long = 0x0001;
pub const VisualScreenMask: c_long = 0x0002;
pub const VisualDepthMask: c_long = 0x0004;
pub const VisualClassMask: c_long = 0x0008;
pub const VisualRedMaskMask: c_long = 0x0010;
pub const VisualGreenMaskMask: c_long = 0x0020;
pub const VisualBlueMaskMask: c_long = 0x0040;
pub const VisualColormapSizeMask: c_long = 0x0080;
pub const VisualBitsPerRGBMask: c_long = 0x0100;
pub const VisualAllMask: c_long = 0x01ff;

// window class
pub const InputOutput: c_uint = 1;
pub const InputOnly: c_uint = 2;


//
// local impls
//


impl Default for *mut Display {
  fn default () -> *mut Display {
    std::ptr::null_mut()
  }
}

impl Default for *const Screen {
  fn default () -> *const Screen {
    std::ptr::null()
  }
}

impl Default for *const Visual {
  fn default () -> *const Visual {
    std::ptr::null()
  }
}

