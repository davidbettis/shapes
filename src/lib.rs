// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

// Define the size of our "checkerboard"
const SIZE: usize = 500;

/*
 * 1. What is going on here?
 * Create a static mutable byte buffer.
 * We will use for putting the output of our graphics,
 * to pass the output to js.
 * NOTE: global `static mut` means we will have "unsafe" code
 * but for passing memory between js and wasm should be fine.
 *
 * 2. Why is the size SIZE * SIZE * 4?
 * We want to have SIZExSIZE pixels. And 4 colors per pixel (r,g,b,a)
 * Which, the Canvas API Supports.
 */
const OUTPUT_BUFFER_SIZE: usize = SIZE * SIZE * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
  let pointer: *const u8;
  unsafe {
    pointer = OUTPUT_BUFFER.as_ptr();
  }

  return pointer;
}

pub fn draw_rectangle(
    origin_x: usize,
    origin_y: usize,
    width: usize,
    height: usize,
    color_red: u8,
    color_green: u8,
    color_blue: u8
    ) {
  for y in origin_y..origin_y+height {
    for x in origin_x..origin_x+width {
      let square_number: usize = y * SIZE + x;
      let square_rgba_index: usize = square_number * 4;

      unsafe {
        OUTPUT_BUFFER[square_rgba_index + 0] = color_red; // Red
        OUTPUT_BUFFER[square_rgba_index + 1] = color_green; // Green
        OUTPUT_BUFFER[square_rgba_index + 2] = color_blue; // Blue
        OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
      }
    }
  }
}

pub fn draw_triangle(
    left_x: usize,
    left_y: usize,
    width: usize,
    height: usize,
    color_red: u8,
    color_green: u8,
    color_blue: u8
    ) {
  // TODO
}

pub fn draw_ellipse(
    center_x: usize,
    center_y: usize,
    width: usize,
    height: usize,
    color_red: u8,
    color_green: u8,
    color_blue: u8
    ) {
  // TODO
}

// Function to draw random cascading shapes on the screen.
#[wasm_bindgen]
pub fn draw_shapes() {
  let mut rng = thread_rng();
  let x = rng.gen_range(0, SIZE as u8) as usize;
  let y = rng.gen_range(0, SIZE as u8) as usize;
  let width = rng.gen_range(10, 100);
  let height = rng.gen_range(10, 100);
  let color_r = rng.gen_range(64, 255);
  let color_g = rng.gen_range(64, 255);
  let color_b = rng.gen_range(64, 255);
  let color_b = rng.gen_range(0, 255);
  draw_rectangle(x, y, width, height, color_r, color_g, color_b);
  draw_rectangle(x, y, width, height, color_r, color_g, color_b);
  draw_rectangle(x, y, width, height, color_r, color_g, color_b);
  draw_rectangle(x, y, width, height, color_r, color_g, color_b);
  draw_rectangle(x, y, width, height, color_r, color_g, color_b);
}

