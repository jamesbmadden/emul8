/**
 * This struct reads and interprets instructions, handling memory and connecting with
 * the display and audio outputs as well as the keyboard inputs
 */
use crate::{display::Display, keyboard::Keyboard};

pub struct Cpu {

  // references to the structs that handle input/output
  pub display: Display,
  pub keyboard: Keyboard,

  // 4096 bytes of memory, each byte as a u8
  pub memory: [u8; 4096],
  // 16 8-bit registers
  pub v: [u8; 16],
  // address in memory
  pub memory_addr: usize,
  // address in the program instructions
  pub program_addr: usize,
  // timers for keeping track of delay & sound length
  pub delay_timer: u16,
  pub sound_timer: u16,

  // state for how the game is running
  pub paused: bool,
  pub speed: u16,

  pub stack: Vec<u16>

}

impl Cpu {

  /**
   * Create all the necessary data for the cpu, and create instances of each input/output struct
   */
  pub async fn new(window: &winit::window::Window) -> Self {

    // create an instance of display
    let display = Display::new(window).await;
    let keyboard = Keyboard::new();

    // create the memory
    let memory: [u8; 4096] = [0; 4096];
    let v: [u8; 16] = [0; 16];
    let memory_addr: usize = 0;

    // and the timers
    let delay_timer: u16 = 0;
    let sound_timer: u16 = 0;

    // address in the program
    let program_addr: usize = 0;

    let stack: Vec<u16> = vec![];

    // state for how the game is running
    let speed: u16 = 10;
    let paused = false;


    return Cpu { display, keyboard, memory, memory_addr, program_addr, v, delay_timer, sound_timer, stack, speed, paused };

  }

  /**
   * chip-8 contains 16 sprites loaded into the interpreter part of the memory. This function loads them in
   */
  pub fn load_sprites_to_memory(&mut self) {

    // create the definition of all the sprites
    // each sprite is 5 bytes long
    let sprites: [u8; 80] = [
      0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
      0x20, 0x60, 0x20, 0x20, 0x70, // 1
      0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
      0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
      0x90, 0x90, 0xF0, 0x10, 0x10, // 4
      0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
      0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
      0xF0, 0x10, 0x20, 0x40, 0x40, // 7
      0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
      0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
      0xF0, 0x90, 0xF0, 0x90, 0x90, // A
      0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
      0xF0, 0x80, 0x80, 0x80, 0xF0, // C
      0xE0, 0x90, 0x90, 0x90, 0xE0, // D
      0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
      0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    // load these into the system's memory
    for (i, byte) in sprites.into_iter().enumerate() {

      // move the sprite into the same spot in our general memory
      self.memory[i] = byte;

    }

  }

  /**
   * Load the data from a ROM into the system's memory, starting from spot 0x200 as the spec defines
   */
  pub fn load_program_to_memory(&mut self, bytes: Vec<u8>) {

    // iterate over the bytes, and add them to memory, starting from 0x200
    for (i, byte) in bytes.into_iter().enumerate() {

      self.memory[0x200 + i] = byte;

    }

  }

  /**
   * cycle runs 60 times per second, executing instructions
   */
  pub fn cycle(&mut self) {

    // only run certain functions if the system is unpaused
    if !self.paused {

      // run however many instructions are specified in the speed variable
      for i in 0..self.speed {

          // figure out the operation we're running
          let opcode = (self.memory[self.program_addr] as u16) << 8 | self.memory[self.program_addr + 1] as u16;

      }

      // update the timers
      self.update_timers();

    }

    // cause a new render
    // update the visual data and then render
    self.display.update();
    self.display.render();

  }

  /**
   * If the timers are not equal to 0, lower their value by 1 per cycle
   */
  pub fn update_timers(&mut self) {

  }

}