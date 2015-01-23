// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use ::display::Display;
use ::event::{
  ClientMessageData,
  Event,
};


#[test]
fn test () {
  // open display
  let mut display;
  if let Some(d) = Display::open_default() {
    display = d;
  } else {
    panic!("can't open display");
  }

  // get atoms
  let atom_wm_delete_window = display.intern_atom("WM_DELETE_WINDOW");
  let atom_wm_protocols = display.intern_atom("WM_PROTOCOLS");

  // create window
  let screen_num = display.default_screen();
  let root = display.root_window(screen_num);
  let border = display.black_pixel(screen_num);
  let background = display.white_pixel(screen_num);

  let window = display.create_simple_window(root, 0, 0, 640, 480, 0, border, background);

  display.store_name(window, "Xlib Test");
  if !display.set_wm_protocols(window, &[atom_wm_delete_window]) {
    panic!("can't set WM protocols");
  }

  display.map_window(window);

  // main loop
  loop {
    match display.next_event() {
      Event::ClientMessage(e) => {
        if e.message_type == atom_wm_protocols {
          if let ClientMessageData::Long(data) = e.data {
            if data[0] == atom_wm_delete_window {
              break;
            }
          }
        }
      },
//      _ => {},
    }
  }
}
