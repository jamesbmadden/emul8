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
  pub delay_timer: u8,
  pub sound_timer: u8,

  // state for how the game is running
  pub paused: bool,
  pub speed: u16,

  pub stack: Vec<usize>

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
    let delay_timer: u8 = 0;
    let sound_timer: u8 = 0;

    // address in the program
    let program_addr: usize = 0x200;

    let stack: Vec<usize> = vec![];

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

    // run however many instructions are specified in the speed variable
    for _i in 0..self.speed {

      // only run certain functions if the system is unpaused
      if !self.paused && !self.keyboard.awaiting_keypress {

        // if we just resumed from a keyboard-awaiting pause, write that keypress down
        if self.keyboard.handle_resume {
          self.handle_resume();
        }

        // figure out the operation we're running
        let instruction = (self.memory[self.program_addr] as u16) << 8 | self.memory[self.program_addr + 1] as u16;
        // execute the instruction
        self.execute_instruction(instruction);
      }

    }

    // only run if unpaused
    if !self.paused && !self.keyboard.awaiting_keypress {
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

    // if the delay timer is greater than 0, make it smaller :)
    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }
    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }

  }

  /**
   * When resuming from a wait for key, that key must be stored in the location specified in the instruction
   */
  pub fn handle_resume(&mut self) {

    // find the instruction tht induced the pause
    let instruction = (self.memory[self.program_addr] as u16) << 8 | self.memory[self.program_addr + 1] as u16;

    // find the x position to store the last keypress
    let x = ((instruction & 0x0F00) >> 8) as usize;

    // finally, write the most recent keypress to v[x]
    self.v[x] = self.keyboard.latest_key;

  }

  /**
   * The big one: take in an instruction code and determine what to do
   */
  pub fn execute_instruction(&mut self, instruction: u16) {

    // update program address so the next instruction is run
    self.program_addr += 2;

    // x and y values, located at 0xy0 in the instruction, are used in
    // many different instructions, so they should be grabbed now to
    // reduce repetition of code
    // use the & operation to isolate only the specific 4 bits we want
    let x = ((instruction & 0x0F00) >> 8) as usize;
    let y = ((instruction & 0x00F0) >> 4) as usize;

    // match different instructions, starting off with the greatest value (the first 4 bits)
    // and then narrowing down depending on the instruction
    // instructions can be referenced in the technical specification, simple
    // explainers will be included in comments here
    match instruction & 0xF000 {

      // there's two options for what a 0x0 instruction could be
      0x0000 => match instruction {

        // clear the screen
        0x00E0 => self.display.clear(),
        // exit a subroutine by setting the program counter to the top of the stack
        0x00EE => self.program_addr = self.stack.pop().unwrap() as usize,
        // nothing else is real so it can be safely ignored
        _ => ()

      }

      // jump to address nnn where nnn is the last 12 bits in the instruction
      0x1000 => self.program_addr = (instruction & 0x0FFF) as usize,

      // add the current address to the stack and start a subroutine at the last 12 bits
      0x2000 => {
        self.stack.push(self.program_addr);
        self.program_addr = (instruction & 0x0FFF) as usize;
      },

      // if the value of v[x] equals the second byte, skip the next instruction
      0x3000 => if self.v[x] as u16 == instruction & 0x00FF {
        self.program_addr += 2;
      },

      // if the value of v[x] DOESN'T equal the second byte, skip the next instruction
      0x4000 => if self.v[x] as u16 != instruction & 0x00FF {
        self.program_addr += 2;
      },

      // if v[x] equals v[y], skip the next instruction
      0x5000 => if self.v[x] == self.v[y] {
        self.program_addr += 2;
      },

      // set v[x] to the second byte
      0x6000 => self.v[x] = (instruction & 0x00FF) as u8,

      // add the second byte to v[x]
      // make sure it overflows gracefully using bitwise operators
      0x7000 => self.v[x] = ((self.v[x] as u16 + (instruction & 0xFF)) & 0xFF) as u8,


      // 0x8000 series performs operations on the contents of the v store
      // all use x and y values, specify different operation using the last
      // four bits
      0x8000 => match instruction & 0xF {

        // sets store x to the value of store y
        0x0 => self.v[x] = self.v[y],
        // store bitwise OR on v[x] and v[y] in v[x]
        0x1 => self.v[x] = self.v[x] | self.v[y],
        // store bitwise AND on v[x] and v[y] in v[x]
        0x2 => self.v[x] = self.v[x] & self.v[y],
        // store bitwise XOR on v[x] and v[y] in v[x]
        0x3 => self.v[x] = self.v[x] ^ self.v[y],
        // add v[x] and v[y] together, storing extra bit in v[0xF]
        0x4 => {
          // add them together
          let sum = self.v[x] as u16 + self.v[y] as u16;
          // set v[15] to whether or not sum is greater than the max
          self.v[15] = (sum > 255) as u8;
          // set v[x] to the sum, cutting off any potential overflow
          self.v[x] = (sum & 0xFF) as u8;
        },
        // v[y] is subtracted from v[x]. v[15] = v[y] > v[x]
        0x5 => {
          let difference = self.v[x] as i16 - self.v[y] as i16;
          self.v[15] = (self.v[x] > self.v[y]) as u8;
          self.v[x] = (difference & 0xFF) as u8;
        },
        // divide v[x] by 2, and set v[15] to the most significant bit of v[x]
        0x6 => {
          // if v[x] is >= 128, the 8th bit must be 1
          self.v[15] = (self.v[x] >= 128) as u8;
          self.v[x] /= 2;
        },
        // v[x] is subtracted from v[y]. v[15] = v[x] > v[y]
        0x7 => {
          let difference = self.v[y] as i16 - self.v[x] as i16;
          self.v[15] = (self.v[y] > self.v[x]) as u8;
          self.v[x] = (difference & 0xFF) as u8;
        },
        // multiply v[x] by 2, and set v[15] to the most significant bit of v[x]
        0xE => {
          let product = self.v[x] as u16 * 2;
          // if v[x] is >= 128, the 8th bit must be 1
          self.v[15] = (self.v[x] >= 128) as u8;
          self.v[x] = (product & 0xFF) as u8;
        },

        // no other options
        _ => ()

      },

      // skip next instruction if v[x] DOESN'T equal v[y]
      0x9000 => if self.v[x] != self.v[y] {
        self.program_addr += 2;
      },

      // set the i store (memory_addr) to the last 12 bits of the instruction
      0xA000 => self.memory_addr = instruction as usize & 0xFFF,

      // the program counter (program_addr) is set to the last 12 bits + v[0]
      0xB000 => self.program_addr = (instruction as usize & 0xFFF) + self.v[0] as usize,

      // a random number between 0 and 255 is generated and ANDed with kk, then stored in v[x]
      // where 0xCxkk
      0xC000 => {
        let kk = (instruction & 0xFF) as u8;
        let random: u8 = rand::random();
        // now store it
        self.v[x] = random & kk;
      },

      // read sprite from memory at position i (memory_addr) for n bytes, rendering starting at x, y
      // formatted as 0xDxyn
      // the sprite XOR's onto the display, in simple words meaning it flips the state where the bit = 1
      0xD000 => {

        // get the length of bytes, which is the last 4 bits in the instruction
        let n = instruction as usize & 0xF;
        // whether or not a pixel was turned off (that needs to be stored in memory later)
        let mut turned_off = false;

        // run through the bytes, which make up rows
        for row in 0..n {

          // grab the byte
          let mut byte = self.memory[self.memory_addr + row];
          // and now each bit, which make up the columns
          for col in 0..8 {

            // if the bit at the end is NOT zero, change the pixel!
            if (byte & 0x80) > 0 {
              // also keep track of whether a pixel was changed here
              turned_off = turned_off || self.display.set_pixel(self.v[x] as i32 + col, self.v[y] as i32 + row as i32);

              // shift the byte over by one to the left to move the next column to first
              byte = byte << 1;
            }

          }

        }

        // finally, store whether a pixel was turned off in v[15]
        self.v[15] = turned_off as u8;

      },

      // there's two options here
      0xE000 => match instruction & 0xFF {

        // skip next instruction if key stored in v[x] is pressed
        0x9E => if self.keyboard.is_key_pressed(self.v[x]) {
          self.program_addr += 2;
        },
        // skip next instruction if key stored in v[x] ISN'T pressed
        0xA1 => if !self.keyboard.is_key_pressed(self.v[x]) {
          self.program_addr += 2;
        }

        // no other options
        _ => ()

      },

      // there's nine options here
      0xF000 => match instruction & 0xFF {

        // put the value of the delay timer into v[x]
        0x07 => self.v[x] = self.delay_timer,

        // pause execution until a key is pressed
        0x0A => {
          // keyboard.rs handles this, simply just pause execution
          self.keyboard.awaiting_keypress = true;
        },

        // set delay timer to v[x]
        0x15 => self.delay_timer = self.v[x],

        // set sound timer to v[x]
        0x18 => self.sound_timer = self.v[x],

        // add v[x] to i (memory_addr)
        0x1E => self.memory_addr += self.v[x] as usize,

        // i (memory_addr) is set to the address for the sprite x,
        // which is v[x] * 5 since the sprites start from position 0 and
        // sprites are 5 bytes long
        0x29 => self.memory_addr = self.v[x] as usize * 5,

        // store the decimal digits of v[x] in memory locations i, i+1, and i+2
        0x33 => {
          // hundreds digit
          self.memory[self.memory_addr] = self.v[x] / 100;
          // tens digit - first eliminate the ones digit then the hundreds
          self.memory[self.memory_addr + 1] = (self.v[x] / 10) % 10;
          // finally, the ones digit
          self.memory[self.memory_addr + 2] = self.v[x] % 10;
        },

        // store v[0] through v[x] in memory, starting at memory_addr
        0x55 => for i in 0..(x + 1) {

          self.memory[self.memory_addr + i] = self.v[i];

        },

        // read v[0] through v[15] from memory, starting at memory_addr
        0x65 => for i in 0..(x + 1) {

          self.v[i] = self.memory[self.memory_addr + i];

        },

        // no other options
        _ => ()

      },

      // if any instruction is encountered that isn't yet implemented, give a todo
      _ => {
        println!("Instruction {} couldn't be processed", instruction);
        todo!();
      }

    }

  }

}