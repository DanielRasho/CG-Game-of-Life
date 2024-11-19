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
    const CELL_SIZE: usize = 20;
    let framebuffer_width =  window_width;
    let framebuffer_height = window_height;

    let rle = "....................O......O...........
..................OOO....OOO...........
.................O......O..............
.................OO.....OO.............
.....O............................OO...
......OO............................O..
....OO...........................O.....
......O...........................OO...
.......................................
....O.........O...O....................
....O.........O.O.O.O...........OOO....
....O.........O...O....................
.......................................
...O............................O......
..OO..........................O.O......
..OO...........................O.O.....
..O............................O.......
............OO.....OO..................
.............O......O..................
..........OOO....OOO...................
..........O......O.....................";
    
    
    let mut model = match parse_rle_body(rle){
        Ok(v) => v,
        Err(e) => panic!("cant parse rle {e}")
    };
    
    // Frame Rate
    let frame_delay = Duration::from_millis(100);
  
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

    // Split the input string into lines
    let lines: Vec<&str> = rle.lines().collect();

    // Iterate over each line and character
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                'O' => {
                    living_cells.insert(Cell { x, y });
                }
                '.' => {
                    // Dead cells are ignored
                }
                _ => {
                    // Return an error if an invalid character is found
                    return Err(format!("Invalid character '{}' at line {}, column {}", char, y + 1, x + 1));
                }
            }
        }
    }

    // Determine the width and height of the grid
    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    Ok(State {
        width,
        height,
        living_cells,
    })
}
