/**
  Renders the actual state to the screen :)
*/
use winit::window::Window;
use std::borrow::Cow;
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

// resolution of the display
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

// the vertices that make up a single pixel
// basically we're gonna have a bunch of instances of this to fill the screen :)
const PIXEL_VERTICES: [f32; 12] = [
  // first triangle: top left -> bottom left -> top right
  0.0,  1.0,
  0.0, 0.0,
  1.0,  1.0,
  // second triangle: bottom left -> bottom right -> top right
  0.0, 0.0,
  1.0, 0.0,
  1.0,  1.0
];

/**
 * Represents an instance of a pixel to render
 */
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Instance {
  pub pos: [u32; 2],
  pub on: u32
}

/**
 * Display represents both all the visual data and the wgpu instances
 */
pub struct Display {
  pub pixels: [[bool; WIDTH]; HEIGHT], // the state of each pixel on the screen

  // now all the wgpu stuff
  pub surface: wgpu::Surface,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub render_pipeline: wgpu::RenderPipeline,
  pub config: wgpu::SurfaceConfiguration,
  pub vertex_buffer: wgpu::Buffer,
  pub instance_buffer: wgpu::Buffer
}

impl Display {

  /**
   * Create an instance of the display. This both sets up the pixels array and wgpu for rendering
   */
  pub async fn new(window: &Window) -> Self {

    // create an array of pixels, all starting off false
    let pixels = [[false; WIDTH]; HEIGHT];

    // create a wgpu instance! let's get going
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::Backends::all());

    let surface = unsafe { instance.create_surface(&window) };

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      force_fallback_adapter: false,
      // make sure the adapter is compatible with the surface
      compatible_surface: Some(&surface)
    }).await.expect("Couldn't find a compatible adapter :/");

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
      label: None,
      features: wgpu::Features::empty(),
      limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
    }, None).await.expect("Couldn't get the device :(");

    // load the shaders
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl")))
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[],
      push_constant_ranges: &[]
    });

    let swapchain_format = surface.get_supported_formats(&adapter)[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[
          wgpu::VertexBufferLayout {
            array_stride: 2 * 4, // two instances of 4-byte floats
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2]
          },
          wgpu::VertexBufferLayout {
            array_stride: 3 * 4,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![1 => Uint32x2, 2 => Uint32]
          }
        ]
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(swapchain_format.into())]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None
    });

    let mut config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: swapchain_format,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo
    };

    surface.configure(&device, &config);

    // create the vertex buffer
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Buffer"),
      contents: bytemuck::bytes_of(&PIXEL_VERTICES),
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
    });

    // generate the list of instances
    let instances = Display::gen_instances();
    // and make an instance buffer
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Instance Buffer"),
      contents: bytemuck::bytes_of(&instances),
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
    });

    // return an instance of Display using all the variables created
    return Display { pixels, surface, device, queue, render_pipeline, config, vertex_buffer, instance_buffer };

  }

  // generate a list of instances of the pixels to render
  pub fn gen_instances() -> [Instance; WIDTH * HEIGHT] {

    let mut instances: [Instance; WIDTH * HEIGHT] = [Instance {pos: [0, 0], on: 0}; WIDTH * HEIGHT];

    // loop through every tile and generate an instance for that position
    for y in 0..HEIGHT {

      for x in 0..WIDTH {

        // create the instance
        instances[y * WIDTH + x] = Instance {
          pos: [x as u32, y as u32],
          on: 0
        }

      }

    };

    return instances;

  }

  // update the display
  pub fn set_pixel(&mut self, x: i32, y: i32) -> bool {

    // chip8 coords wrap around if negative
    // unsigned integer versions must be used so coordinates work right
    let ux: usize;
    let uy: usize;
    if x < 0 {
      ux = (x + WIDTH as i32) as usize;
    } else if x > WIDTH as i32 {
      ux = (x - WIDTH as i32) as usize;
    } else {
      ux = x as usize;
    }
    if y < 0 {
      uy = (y + HEIGHT as i32) as usize;
    } else if y > HEIGHT as i32 {
      uy = (y - HEIGHT as i32) as usize;
    } else {
      uy = y as usize;
    }

    // set the pixel to whatever it currently isn't
    self.pixels[uy][ux] = !self.pixels[uy][ux];

    // return whether the pixel was erased (which equals the inverse of what it was just set to)
    return !self.pixels[uy][ux];

  }

  // completely clear the screen
  pub fn clear(&mut self) {

    // set every pixel value to false (empty)
    self.pixels = [[false; WIDTH]; HEIGHT];

  }

  // render will actually paint the pixels ooh that's WGPU time
  pub fn render(&self) {
    
    let frame = self.surface.get_current_texture().expect("Couldn't get the current texture");
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {

      let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: true
          }
        })],
        depth_stencil_attachment: None
      });

      pass.set_pipeline(&self.render_pipeline);
      pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
      pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
      pass.draw(0..6, 0..(WIDTH as u32 * HEIGHT as u32));

    }

    self.queue.submit(Some(encoder.finish()));
    frame.present();

  }

}