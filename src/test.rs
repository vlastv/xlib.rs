// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use ::display::{
  Atom,
  Display,
};
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
  let atom_wm_delete_window = force_intern_atom(&mut display, "WM_DELETE_WINDOW");
  let atom_wm_protocols = force_intern_atom(&mut display, "WM_PROTOCOLS");

  // create window
  let screen_num = display.default_screen();
  let root = display.root_window(screen_num);
  let border = display.black_pixel(screen_num);
  let background = display.white_pixel(screen_num);
  let window = display.create_simple_window(root, 0, 0, 640, 480, 0, border, background);
  display.store_name(window, "Xlib Test");
  display.set_wm_protocols(window, &[atom_wm_delete_window]);
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
      _ => {},
    }
  }
}

fn force_intern_atom (display: &mut Display, name: &str) -> Atom {
  if let Some(atom) = display.intern_atom(name, false) {
    return atom;
  } else {
    panic!("failed to retrieve atom: {}", name);
  }
}
