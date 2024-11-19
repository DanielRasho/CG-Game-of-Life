mod internal;

use minifb::{Window, WindowOptions, Key};

use std::time::Duration;

use internal::framebuffer::{self, Framebuffer};
use internal::color::Color;
use internal::render::{calculate_next_generation, render, Cell, State};
use std::collections::HashSet;


pub fn start() {
    // Window Size configuration
    let window_width = 800;
    let window_height = 600;
    const CELL_SIZE: usize = 10;
    let framebuffer_width =  window_width;
    let framebuffer_height = window_height;

    let rle = "18bo6bo$16b3o4b3o$15bo6bo$15b2o5b2o$3bo28b2o$4b2o28bo$2b2o27bo$4bo27b
2o2$2bo9bo3bo$2bo9bobobobo11b3o$2bo9bo3bo2$bo28bo$2o26bobo$2o27bobo$o
28bo$10b2o5b2o$11bo6bo$8b3o4b3o$8bo6bo!";
    
    
    let mut model = match parse_rle_body(rle){
        Ok(v) => v,
        Err(e) => panic!("cant parse rle {e}")
    };
    
    // Frame Rate
    let frame_delay = Duration::from_millis(3000);
  
    // Window Objects initialization
    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::new(0, 0, 0));
    let mut window = Window::new(
      "Game of Life",
      window_width,
      window_height,
      WindowOptions::default()
    ).unwrap();
    
    
    framebuffer.set_background_color(Color::new(30, 20, 120));
    
    
    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        framebuffer.clear();
        framebuffer.set_current_color_hex(0xFFFFFF);
        
        
        render(&mut framebuffer, &mut model, CELL_SIZE);
        
        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        calculate_next_generation(&mut model);

        std::thread::sleep(frame_delay)
    }
}

pub fn parse_rle_body(rle: &str) -> Result<State, String> {
    let mut living_cells = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    let mut count = 0;

    for c in rle.chars() {
        match c {
            'b' => {
                // Skip 'count' dead cells
                x += count.max(1);
                count = 0;
            }
            'o' => {
                // Add 'count' live cells
                for _ in 0..count.max(1) {
                    // println!("Cell at : {x}, {y}");
                    living_cells.insert(Cell { x, y });
                    x += 1;
                }
                count = 0;
            }
            '$' => {
                // End the row, move to the next row
                y += 1;
                x = 0;
            }
            '!' => break, // End of RLE
            '\n' => continue, // End of RLE
            '0'..='9' => {
                // Accumulate count for the next cells
                count = count * 10 + c.to_digit(10).unwrap() as usize;
            }
            _ => return Err(format!("Invalid character in RLE: '{}'", c)),
        }
    }

    Ok(State { width: 70, height: 70, living_cells })
}
