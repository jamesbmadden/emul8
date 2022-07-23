mod display;
mod cpu;
mod keyboard;

use winit::{
  event::{Event, WindowEvent, KeyboardInput, ElementState},
  dpi::LogicalSize,
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use std::{fs::File, io::Read, time::{Duration, Instant}};

/**
 * wgpu and winit require asynchronous features to run, so using a seperate function
 * makes most sense
 */
async fn run() {

  // define the window's properties
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().with_title("emul8 😏").with_inner_size(LogicalSize::new(600, 300)).build(&event_loop).unwrap();

  // create an instance of the display for rendering 
  let mut cpu = cpu::Cpu::new(&window).await;
  
  // load the sprites into memory
  cpu.load_sprites_to_memory();

  // load the ROM into storage
  let mut program_bytes: Vec<u8> = Vec::new();
  // open the file and grab the bytes into the program_bytes vector
  let mut rom = File::open("roms/BLINKY").unwrap();
  rom.read_to_end(&mut program_bytes).expect("Failed to load the rom, is it missing?");
  // finally, pass the bytes to cpu to load into memory
  cpu.load_program_to_memory(program_bytes);

  // run a cycle (for testing)
  // this will need to move to a 60x per second loop soon
  cpu.cycle();

  // keep track of timing so that a cpu cycle can be executed every 16.667 milliseconds
  let mut prev_cycle = Instant::now();

  // open up the window!
  event_loop.run(move | event, _, control_flow | {

    // make sure window stays open until the close event
    *control_flow = ControlFlow::Poll;

    // If it's been a 60th of a second, run a cpu cycle
    if Instant::now() > prev_cycle + Duration::from_micros(16667) {

      // now that it's time for a new cycle, reset the previous cycle time
      // this happens BEFORE the cycle so that the execution time of cycle doesn't affect
      // how long it takes for a new frame
      prev_cycle = Instant::now();

      // actually run the cycle!
      cpu.cycle();

    }

    // handle events
    match event {

      // render the window!
      Event::RedrawRequested(..) => {

        // a redraw means an event such as scaling, so a simple render (NOT a cycle!) will do
        cpu.display.render();

      },

      // close the window
      Event::WindowEvent { 
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,

      // key pressed or released!
      Event::WindowEvent {
        event: WindowEvent::KeyboardInput { 
          input: KeyboardInput { state, virtual_keycode, .. }, 
          .. 
        },
        ..
      } => {

        // only go if there's actually a virtual keycode
        if virtual_keycode != None {
          // connect with the keyboard struct
          if state == ElementState::Pressed {

            // key is pressed, run on_key_down
            cpu.keyboard.on_key_down(virtual_keycode.unwrap());
            
          }

          else if state == ElementState::Released {
            // key is released, run on_key_up
            cpu.keyboard.on_key_up(virtual_keycode.unwrap());
          }
        }

      }

      // catchall, do nothing
      _ => {}

    }

  });

}
fn main() {
  
  // WASM needs a canvas created and appended, implement that when WASM support
  // is implemented in the future
  #[cfg(not(target_arch = "wasm32"))]
  {
    env_logger::init();
    // run the asynchronous run function until completion
    pollster::block_on(run());
  }

}
