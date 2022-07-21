/**
 * Handles user input through the keyboard.
 */
use std::collections::HashMap;
use winit::event::VirtualKeyCode;

pub struct Keyboard {

  // map of keys from wgpu to numbers for the instructions to process
  KEY_MAP: HashMap<VirtualKeyCode, u8>

}

impl Keyboard {

  pub fn new() -> Self {

    // create the key map
    // this maps the modern key arrangement to the traditional chip-8 key codes
    let KEY_MAP: HashMap<VirtualKeyCode, u8> = HashMap::from([
      (VirtualKeyCode::Key1,  0x1),
      (VirtualKeyCode::Key2,  0x2),
      (VirtualKeyCode::Key3,  0x3),
      (VirtualKeyCode::Key4,  0xC),
      (VirtualKeyCode::Q,     0x4),
      (VirtualKeyCode::W,     0x5),
      (VirtualKeyCode::E,     0x6),
      (VirtualKeyCode::R,     0xD),
      (VirtualKeyCode::A,     0x7),
      (VirtualKeyCode::S,     0x8),
      (VirtualKeyCode::D,     0x9),
      (VirtualKeyCode::F,     0xE),
      (VirtualKeyCode::Z,     0xA),
      (VirtualKeyCode::X,     0x0),
      (VirtualKeyCode::C,     0xB),
      (VirtualKeyCode::V,     0xF)
    ]);

    return Keyboard { KEY_MAP };

  }

}