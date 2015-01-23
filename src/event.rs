// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use std::mem::{
  size_of,
  zeroed,
};
use std::ptr::null_mut;

use libc::{
  c_char,
  c_int,
  c_long,
  c_short,
  c_ulong,
};

use ::display::Atom;
use ::internal::{
  FromNative,
  ToNative,
};
use ::window::Window;


//
// ClientMessageData
//


#[derive(Clone, Copy)]
pub enum ClientMessageData {
  Byte([u8; 20]),
  Short([u16; 10]),
  Long([u32; 5]),
}


//
// ClientMessageEvent
//


#[derive(Clone, Copy)]
pub struct ClientMessageEvent {
  pub serial: u32,
  pub send_event: bool,
  pub window: Window,
  pub message_type: Atom,
  pub data: ClientMessageData,
}

impl FromNative<::ffi::XClientMessageEvent> for Option<ClientMessageEvent> {
  fn from_native (xclient: ::ffi::XClientMessageEvent) -> Option<ClientMessageEvent> {
    let data;
    match xclient.format {
      8 => {
        let mut array: [u8; 20] = [0; 20];
        for i in 0..20us {
          array[i] = xclient.get_byte(i) as u8;
        }
        data = ClientMessageData::Byte(array);
      },
      16 => {
        let mut array: [u16; 10] = [0; 10];
        for i in 0..10us {
          array[i] = xclient.get_short(i) as u16;
        }
        data = ClientMessageData::Short(array);
      },
      32 => {
        let mut array: [u32; 5] = [0; 5];
        for i in 0..5us {
          array[i] = xclient.get_long(i) as u32;
        }
        data = ClientMessageData::Long(array);
      },
      _ => {
        return None;
      },
    }
    let e = ClientMessageEvent {
      serial: xclient.serial as u32,
      send_event: xclient.send_event != 0,
      window: xclient.window as Window,
      message_type: xclient.message_type as Atom,
      data: data,
    };
    return Some(e);
  }
}

impl ToNative<::ffi::XEvent> for ClientMessageEvent {
  fn to_native (&self) -> ::ffi::XEvent {
    unsafe {
      let mut xclient: ::ffi::XClientMessageEvent = zeroed();
      xclient.kind = ::ffi::ClientMessage;
      xclient.serial = self.serial as c_ulong;
      xclient.send_event = if self.send_event {1} else {0};
      xclient.display = null_mut();
      xclient.window = self.window as c_ulong;
      xclient.message_type = self.message_type as c_ulong;
      match self.data {
        ClientMessageData::Byte(ref array) => {
          for i in 0..20us {
            xclient.set_byte(i, array[i] as c_char);
          }
        },
        ClientMessageData::Short(ref array) => {
          for i in 0..10us {
            xclient.set_short(i, array[i] as c_short);
          }
        },
        ClientMessageData::Long(ref array) => {
          for i in 0..5us {
            xclient.set_long(i, array[i] as c_long);
          }
        },
      }
      return reinterpret(&xclient);
    }
  }
}


//
// DestroyWindowEvent
//


#[derive(Clone, Copy)]
pub struct DestroyWindowEvent {
  pub serial: u32,
  pub send_event: bool,
  pub event: Window,
  pub window: Window,
}

impl FromNative<::ffi::XDestroyWindowEvent> for DestroyWindowEvent {
  fn from_native (xdestroy: ::ffi::XDestroyWindowEvent) -> DestroyWindowEvent {
    DestroyWindowEvent {
      serial: xdestroy.serial as u32,
      send_event: xdestroy.send_event != 0,
      event: xdestroy.event as Window,
      window: xdestroy.window as Window,
    }
  }
}

impl ToNative<::ffi::XEvent> for DestroyWindowEvent {
  fn to_native (&self) -> ::ffi::XEvent {
    unsafe {
      let xdestroy = ::ffi::XDestroyWindowEvent {
        kind: ::ffi::DestroyNotify,
        serial: self.serial as c_ulong,
        send_event: if self.send_event {1} else {0},
        display: null_mut(),
        event: self.event as c_ulong,
        window: self.window as c_ulong,
      };
      return reinterpret(&xdestroy);
    }
  }
}


//
// Event
//


#[derive(Clone, Copy)]
pub enum Event {
  ClientMessage(ClientMessageEvent),
  DestroyWindow(DestroyWindowEvent),
  Expose(ExposeEvent),
}

impl FromNative<::ffi::XEvent> for Option<Event> {
  fn from_native (xevent: ::ffi::XEvent) -> Option<Event> {
    unsafe {
      match xevent.kind() {
        ::ffi::ClientMessage => {
          if let Some(e) = FromNative::from_native(reinterpret(&xevent)) {
            Some(Event::ClientMessage(e))
          } else {
            None
          }
        }
        ::ffi::DestroyNotify => Some(Event::DestroyWindow(FromNative::from_native(reinterpret(&xevent)))),
        ::ffi::Expose => Some(Event::Expose(FromNative::from_native(reinterpret(&xevent)))),
        _ => None,
      }
    }
  }
}

impl ToNative<::ffi::XEvent> for Event {
  fn to_native (&self) -> ::ffi::XEvent {
    match *self {
      Event::ClientMessage(ref e) => e.to_native(),
      Event::DestroyWindow(ref e) => e.to_native(),
      Event::Expose(ref e) => e.to_native(),
    }
  }
}


//
// EventMask
//


#[derive(Clone, Copy, Eq, PartialEq)]
pub struct EventMask {
  pub key_press: bool,
  pub key_release: bool,
  pub button_press: bool,
  pub button_release: bool,
  pub enter_window: bool,
  pub leave_window: bool,
  pub pointer_motion: bool,
  pub pointer_motion_hint: bool,
  pub button_1_motion: bool,
  pub button_2_motion: bool,
  pub button_3_motion: bool,
  pub button_4_motion: bool,
  pub button_5_motion: bool,
  pub button_motion: bool,
  pub keymap_state: bool,
  pub exposure: bool,
  pub visibility_change: bool,
  pub structure_notify: bool,
  pub resize_redirect: bool,
  pub substructure_notify: bool,
  pub substructure_redirect: bool,
  pub focus_change: bool,
  pub property_change: bool,
  pub colormap_change: bool,
  pub owner_grab_button: bool,
}

impl EventMask {
  pub fn empty () -> EventMask {
    EventMask {
      key_press: false,
      key_release: false,
      button_press: false,
      button_release: false,
      enter_window: false,
      leave_window: false,
      pointer_motion: false,
      pointer_motion_hint: false,
      button_1_motion: false,
      button_2_motion: false,
      button_3_motion: false,
      button_4_motion: false,
      button_5_motion: false,
      button_motion: false,
      keymap_state: false,
      exposure: false,
      visibility_change: false,
      structure_notify: false,
      resize_redirect: false,
      substructure_notify: false,
      substructure_redirect: false,
      focus_change: false,
      property_change: false,
      colormap_change: false,
      owner_grab_button: false,
    }
  }
}

impl FromNative<c_long> for EventMask {
  fn from_native (mask: c_long) -> EventMask {
    EventMask {
      key_press: mask & 0x0000_0001 != 0,
      key_release: mask & 0x0000_0002 != 0,
      button_press: mask & 0x0000_0004 != 0,
      button_release: mask & 0x0000_0008 != 0,
      enter_window: mask & 0x0000_0010 != 0,
      leave_window: mask & 0x0000_0020 != 0,
      pointer_motion: mask & 0x0000_0040 != 0,
      pointer_motion_hint: mask & 0x0000_0080 != 0,
      button_1_motion: mask & 0x0000_0100 != 0,
      button_2_motion: mask & 0x0000_0200 != 0,
      button_3_motion: mask & 0x0000_0400 != 0,
      button_4_motion: mask & 0x0000_0800 != 0,
      button_5_motion: mask & 0x0000_1000 != 0,
      button_motion: mask & 0x0000_2000 != 0,
      keymap_state: mask & 0x0000_4000 != 0,
      exposure: mask & 0x0000_8000 != 0,
      visibility_change: mask & 0x0001_0000 != 0,
      structure_notify: mask & 0x0002_0000 != 0,
      resize_redirect: mask & 0x0004_0000 != 0,
      substructure_notify: mask & 0x0008_0000 != 0,
      substructure_redirect: mask & 0x0010_0000 != 0,
      focus_change: mask & 0x0020_0000 != 0,
      property_change: mask & 0x0040_0000 != 0,
      colormap_change: mask & 0x0080_0000 != 0,
      owner_grab_button: mask & 0x0100_0000 != 0,
    }
  }
}

impl ToNative<c_long> for EventMask {
  fn to_native (&self) -> c_long {
    let mut ord: c_long = 0;
    if self.key_press { ord |= 0x0000_0001; }
    if self.key_release { ord |= 0x0000_0002; }
    if self.button_press { ord |= 0x0000_0004; }
    if self.button_release { ord |= 0x0000_0008; }
    if self.enter_window { ord |= 0x0000_0010; }
    if self.leave_window { ord |= 0x0000_0020; }
    if self.pointer_motion { ord |= 0x0000_0040; }
    if self.pointer_motion_hint { ord |= 0x0000_0080; }
    if self.button_1_motion { ord |= 0x0000_0100; }
    if self.button_2_motion { ord |= 0x0000_0200; }
    if self.button_3_motion { ord |= 0x0000_0400; }
    if self.button_4_motion { ord |= 0x0000_0800; }
    if self.button_5_motion { ord |= 0x0000_1000; }
    if self.button_motion { ord |= 0x0000_2000; }
    if self.keymap_state { ord |= 0x0000_4000; }
    if self.exposure { ord |= 0x0000_8000; }
    if self.visibility_change { ord |= 0x0001_0000; }
    if self.structure_notify { ord |= 0x0002_0000; }
    if self.resize_redirect { ord |= 0x0004_0000; }
    if self.substructure_notify { ord |= 0x0008_0000; }
    if self.substructure_redirect { ord |= 0x0010_0000; }
    if self.focus_change { ord |= 0x0020_0000; }
    if self.property_change { ord |= 0x0040_0000; }
    if self.colormap_change { ord |= 0x0080_0000; }
    if self.owner_grab_button { ord |= 0x0100_0000; }
    return ord;
  }
}


//
// ExposeEvent
//


#[derive(Clone, Copy)]
pub struct ExposeEvent {
  pub serial: u32,
  pub send_event: bool,
  pub window: Window,
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub count: i32,
}

impl FromNative<::ffi::XExposeEvent> for ExposeEvent {
  fn from_native (xexpose: ::ffi::XExposeEvent) -> ExposeEvent {
    ExposeEvent {
      serial: xexpose.serial as u32,
      send_event: xexpose.send_event != 0,
      window: xexpose.window as Window,
      x: xexpose.x as i32,
      y: xexpose.y as i32,
      width: xexpose.width as i32,
      height: xexpose.height as i32,
      count: xexpose.count as i32,
    }
  }
}

impl ToNative<::ffi::XEvent> for ExposeEvent {
  fn to_native (&self) -> ::ffi::XEvent {
    unsafe {
      let xexpose = ::ffi::XExposeEvent {
        kind: ::ffi::Expose,
        serial: self.serial as c_ulong,
        send_event: if self.send_event {1} else {0},
        display: null_mut(),
        window: self.window as c_ulong,
        x: self.x as c_int,
        y: self.y as c_int,
        width: self.width as c_int,
        height: self.height as c_int,
        count: self.count as c_int,
      };
      return reinterpret(&xexpose);
    }
  }
}


//
// private functions
//


unsafe fn reinterpret<I, O> (input: &I) -> O
  where I: Copy + Sized, O: Copy + Sized
{
  if size_of::<I>() >= size_of::<O>() {
    return *(input as *const I as *const O);
  } else {
    let mut output: O = zeroed();
    *(&mut output as *mut O) = *(input as *const I as *const O);
    return output;
  }
}
