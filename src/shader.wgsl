struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) on: u32
};

@vertex
fn vs_main(@location(0) vpos: vec2<f32>, @location(1) ipos: vec2<u32>, @location(2) on: u32) -> VertexOutput {
  // vpos is the vertex position, ipos is the instance position, on is whether or not this tile is illuminated

  var output: VertexOutput;
  var width: f32 = 64.0;
  var height: f32 = 32.0;
  // size of the tile
  var twidth = 2.0 / width;
  var theight = 2.0 / height;

  // the actual position of this pixel must be determined using instance position relative to the vertex position
  var xbase: f32 = f32(ipos[0]) / width * 2.0 - 1.0;
  var ybase: f32 = f32(ipos[1]) / height * 2.0 - 1.0;

  // and get the position including a vertex adjustment
  var x = xbase + vpos[0] * twidth;
  var y = ybase + vpos[1] * theight;

  output.pos = vec4<f32>(x, y, 0.0, 1.0);

  return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
  // the colour will be on for each rgb value: if on is zero it should be black, and if on is 1 it should be white perfect
  var c: f32 = f32(input.on);

  return vec4<f32>(c, c, c, 1.0);
}
