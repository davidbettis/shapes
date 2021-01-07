// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

extern crate web_sys;

// Define the size of our "checkerboard"
const SIZE: usize = 500;

// 2-d point
#[derive(Copy, Clone)]
pub struct Point {
  x: usize,
  y: usize
}

#[derive(Copy, Clone)]
pub struct Color {
  r: u8, // red
  g: u8, // green
  b: u8 // blue
}

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
    color: Color
    ) {
  for y in origin_y..origin_y+height {
    for x in origin_x..origin_x+width {
      let square_number: usize = y * SIZE + x;
      let square_rgba_index: usize = square_number * 4;

      unsafe {
        OUTPUT_BUFFER[square_rgba_index + 0] = color.r; // Red
        OUTPUT_BUFFER[square_rgba_index + 1] = color.g; // Green
        OUTPUT_BUFFER[square_rgba_index + 2] = color.b; // Blue
        OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
      }
    }
  }
}

// calculate the slope betweent the two points; rise / run
pub fn slope(p1: Point, p2: Point) -> f64 {
    let p1_x_f = p1.x as f64;
    let p1_y_f = p1.y as f64;
    let p2_x_f = p2.x as f64;
    let p2_y_f = p2.y as f64;
    return (p2_y_f - p1_y_f) / (p2_x_f - p1_x_f);
}

pub enum TriangleDirection {
    Up,
    Down
    // TODO implement me
    //Right,
    //Left
}

pub fn draw_triangle(
    left_x: usize,
    left_y: usize,
    width: usize,
    height: usize,
    direction: TriangleDirection,
    color: Color
    ) {

  let p1: Point;
  let p2: Point;
  let p3: Point;

  match direction {
    // (0,0)-origin
    // p1----p3
    // l1\  /l2
    //    \/
    //     p2
    TriangleDirection::Down => {
      p1 = Point { x: 0, y: 0};
      p2 = Point { x: (width / 2) as usize, y: height};
      p3 = Point { x: width, y: 0};
    }
    // (0,0)-origin
    //    p2
    //  l1/\l2
    //   /  \
    // p1----p3
    _ => { // TriangleDirection::Up
      p1 = Point { x: 0, y: height};
      p2 = Point { x: (width / 2) as usize, y: 0 };
      p3 = Point { x: width, y: height };
    },
  }

  //web_sys::console::log_1(&format!("p1: ({}, {}), p2: ({}, {}), p3: ({}, {})", p1.x, p1.y, p2.x, p2.y, p3.x, p3.y).into());

  // calculate the slope of each edge
  let l1_slope = slope(p1, p2);
  let l2_slope = slope(p2, p3);

  // y = mx + b, solve for b
  let l1_b = p1.y as f64 - l1_slope * p1.x as f64;
  let l2_b = p3.y as f64 - l2_slope * p3.x as f64;

  web_sys::console::log_1(&format!("l1_slope: {}, l1_b: {}, l2_slope: {}, l2_b: {}", l1_slope, l1_b, l2_slope, l2_b).into());

  // iterate over every row of the triangle and draw x pixels
  for y in 0 .. height {
    // since we have the slope and the intercept, we can calculate the start & end x-values
    // (y - b)/m = x
    let start_x = ((y as f64 - l1_b as f64)/(l1_slope as f64)) as usize;
    let end_x = ((y as f64 - l2_b as f64)/(l2_slope as f64)) as usize;

    //web_sys::console::log_1(&format!("y: {}, start_x: {}, end_x: {}, diff: {}", y, start_x, end_x, (end_x-start_x)).into());
    for x in start_x..end_x {
      draw_grid(Point {x: x, y: y}, color, Point {x: left_x, y: left_y});
    }
  }
}

pub fn draw_grid(p: Point, color: Color, translation: Point) {
  let square_number: usize = (translation.y + p.y) * SIZE + (translation.x + p.x);
  let square_rgba_index: usize = square_number * 4;
  unsafe {
     OUTPUT_BUFFER[square_rgba_index + 0] = color.r; // Red
     OUTPUT_BUFFER[square_rgba_index + 1] = color.g; // Green
     OUTPUT_BUFFER[square_rgba_index + 2] = color.b; // Blue
     OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
  }
}

/*
// TODO - implement me
pub fn draw_ellipse(
    center_x: usize,
    center_y: usize,
    width: usize,
    height: usize,
    color_red: u8,
    color_green: u8,
    color_blue: u8
    ) {
}
*/

pub fn draw_random_rectangle() {
  let mut rng = thread_rng();
  let x = rng.gen_range(0, SIZE as u8) as usize;
  let y = rng.gen_range(0, SIZE as u8) as usize;
  let width = rng.gen_range(10, 100);
  let height = rng.gen_range(10, 100);
  let color = Color {
    r: rng.gen_range(64, 255),
    g: rng.gen_range(64, 255),
    b: rng.gen_range(64, 255)
  };
  draw_rectangle(x, y, width, height, color);
}

pub fn draw_random_triangle() {
  let mut rng = thread_rng();
  let x = rng.gen_range(0, SIZE as u8) as usize;
  let y = rng.gen_range(0, SIZE as u8) as usize;
  let width = rng.gen_range(10, 100);
  let height = rng.gen_range(10, 100);
  let color = Color {
    r: rng.gen_range(64, 255),
    g: rng.gen_range(64, 255),
    b: rng.gen_range(64, 255)
  };

  let coin = rng.gen_range(0, 2);
  let mut direction = TriangleDirection::Up;
  if coin == 1 {
    direction = TriangleDirection::Down;
  }
  draw_triangle(x, y, width, height, direction, color);
}

// Function to draw random cascading shapes on the screen.
#[wasm_bindgen]
pub fn draw_shapes() {
  draw_random_rectangle();
  draw_random_triangle();
}

