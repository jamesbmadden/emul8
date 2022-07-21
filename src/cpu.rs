/**
 * This struct reads and interprets instructions, handling memory and connecting with
 * the display and audio outputs as well as the keyboard inputs
 */
use crate::display::Display;

pub struct Cpu {

  // references to the structs that handle input/output
  pub display: Display,

  // 4096 bytes of memory, each byte as a u8
  pub memory: [u8; 4096]

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

    return Cpu { display, memory };

  }

}