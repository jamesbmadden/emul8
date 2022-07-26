/**
 * Handles user input through the keyboard.
 */
use std::collections::{HashMap, HashSet};
use winit::event::VirtualKeyCode;

pub struct Keyboard {

  // map of keys from wgpu to numbers for the instructions to process
  pub key_map: HashMap<VirtualKeyCode, u8>,
  pub keys_down: HashSet<u8>,
  // whether or not the execution is paused awaiting next key press
  pub awaiting_keypress: bool,
  // the most recent key press
  pub latest_key: u8,
  // finally, whether or not the cpu has to handle resumption
  pub handle_resume: bool

}

impl Keyboard {

  pub fn new() -> Self {

    // create the key map
    // this maps the modern key arrangement to the traditional chip-8 key codes
    let key_map: HashMap<VirtualKeyCode, u8> = HashMap::from([
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

    // the list of which keys are currently down
    let keys_down: HashSet<u8> = HashSet::new();

    // whether or not we're awaiting a key press before continuing
    let awaiting_keypress = false;
    let handle_resume = false;
    let latest_key = 0;

    return Keyboard { key_map, keys_down, awaiting_keypress, handle_resume, latest_key };

  }

  /**
   * Check whether keys_down contains the requested key
   */
  pub fn is_key_pressed(&self, key_code: u8) -> bool {

    // just check if key_code is in the set
    return self.keys_down.contains(&key_code);

  }

  /**
   * A key is pressed! Add it to the set and potentially run next key press
   */
  pub fn on_key_down(&mut self, key: VirtualKeyCode) {

    // first, find the chip-8 key code
    let key_code = self.key_map.get(&key);

    // if key_code is None, it's not one of our wanted keys so we should return now
    if key_code == None { return };

    // now add the pressed key to the pressed key set
    self.keys_down.insert(*key_code.unwrap());

    // and set that to the latest key press
    self.latest_key = *key_code.unwrap();

    // check whether we need to resume execution of the cpu
    if self.awaiting_keypress == true {
      // set awaiting keypress to false and tell the cpu to process the resume
      self.awaiting_keypress = false;
      self.handle_resume = true;
    }

  }

  /**
   * The key is up, remove it from the set
   */
  pub fn on_key_up(&mut self, key: VirtualKeyCode) {

    // first, find the chip-8 key code
    let key_code = self.key_map.get(&key);

    // if key_code is None, it's not one of our wanted keys so we should return now
    if key_code == None { return };

    // now remove the pressed key frpm the pressed key set
    self.keys_down.remove(key_code.unwrap());

  }

}