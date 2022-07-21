mod display;

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

/**
 * wgpu and winit require asynchronous features to run, so using a seperate function
 * makes most sense
 */
async fn run() {

  // define the window's properties
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().with_title("emul8 😏").build(&event_loop).unwrap();

  // create an instance of the display for rendering 
  let mut display = display::Display::new(&window).await;
  // set some pixels to true for testing
  display.set_pixel(5, 21);
  display.set_pixel(49, 3);

  // open up the window!
  event_loop.run(move | event, _, control_flow | {

    // make sure window stays open until the close event
    *control_flow = ControlFlow::Wait;

    // handle events
    match event {

      // render the window!
      Event::RedrawRequested(..) => {

        // update the visual data and then render
        display.update();
        display.render();

      },

      // close the window
      Event::WindowEvent { 
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,

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
