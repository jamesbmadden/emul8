/**
 * This struct reads and interprets instructions, handling memory and connecting with
 * the display and audio outputs as well as the keyboard inputs
 */
use crate::display::Display;

pub struct Cpu {

  // references to the structs that handle input/output
  pub display: Display,

  // 4096 bytes of memory, each byte as a u8
  pub memory: [u8; 4096],
  // 16 8-bit registers
  pub v: [u8; 16],
  // address in memory
  pub memory_addr: u16,
  // address in the program instructions
  pub program_addr: u16,
  // timers for keeping track of delay & sound length
  pub delay_timer: u16,
  pub sound_timer: u16,

  pub stack: Vec<u16>

}

impl Cpu {

  /**
   * Create all the necessary data for the cpu, and create instances of each input/output struct
   */
  pub async fn new(window: &winit::window::Window) -> Self {

    // create an instance of display
    let mut display = Display::new(window).await;

    // create the memory
    let mut memory: [u8; 4096] = [0; 4096];
    let mut v: [u8; 16] = [0; 16];
    let mut memory_addr: u16 = 0;

    // and the timers
    let mut delay_timer: u16 = 0;
    let mut sound_timer: u16 = 0;

    // address in the program
    let mut program_addr: u16 = 0;

    let mut stack: Vec<u16> = vec![];


    return Cpu { display, memory, memory_addr, program_addr, v, delay_timer, sound_timer, stack };

  }

}