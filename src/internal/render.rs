
use std::arch::x86_64::_CMP_FALSE_OQ;
use std::cell;
use std::collections::HashSet;

use nalgebra_glm::Vec2;

use super::framebuffer::Framebuffer;
use super::color::Color;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub struct Cell {
    pub x: usize,
    pub y: usize
}
pub struct State {
    pub width : usize,
    pub height : usize,
    pub living_cells : HashSet<Cell>
}


pub fn render(framebuffer: &mut Framebuffer, state: &mut State, cell_size: usize) {
    for cell in &state.living_cells {
        let x_coor = cell.x * cell_size + 32;
        let y_coor = cell.y * cell_size + 32;
        // println!("x_coor : {x_coor} y_coor: {y_coor}");
        draw_cell(framebuffer, x_coor, y_coor, cell_size);
    }
}

pub fn draw_cell(framebuffer: &mut Framebuffer, x: usize, y:usize, cell_size: usize){
    for x_cord in x..x+cell_size {
        for y_cord in y..y+cell_size {
            framebuffer.draw_point(x_cord, y_cord, 1.0);
        }
    }
}

pub fn calculate_next_generation(state: &mut State){

    // Borrow `state` immutably for `living_cells`, and mutably for neighbors.
    let living_cells = &state.living_cells;

    // We can directly access `living_cells` without cloning it
    let new_generation = living_cells
        .iter()
        .flat_map(|cell| {
            // We borrow `state` mutably only inside this function call to get neighbors
            let mut neighbors = get_neightbor_cells(state, cell);  
            neighbors.push(Some(*cell));  // Include the cell itself as a neighbor
            neighbors.into_iter()
                .filter_map(|cell| cell)
        }).filter_map(|cell| {
            evaluate_cell(state, &cell)
        })
        .collect::<HashSet<Cell>>();
        // println!("=====================");

    state.living_cells = new_generation;
}
// Checks if a cell 
pub fn evaluate_cell(state: & State, cell: &Cell) -> Option<Cell>{
    
    let neighbors = get_neightbor_cells(state, cell);
    
    let neighbor_living_cells = neighbors
    .iter()
    .filter(|n| match n {
        None => false,
        Some(neighbor) => state.living_cells.contains(neighbor),
    })
    .count();
    
    /*
        for n in neighbors {
            match n {
                Some(cell) => println!("\t x:{} y:{}", cell.x, cell.y),
                None => {},
            }
        }
     */

    let is_alive = state.living_cells.contains(&cell);

    // Conway's Game of Life rules:
    if is_alive {
        if neighbor_living_cells == 2 || neighbor_living_cells == 3{
            // println!("For living cell {} {} is ALIVE", cell.x, cell.y);
            return Some(cell.clone())
        } 
        // println!("For living cell {} {} is DEAD", cell.x, cell.y);
        return None
    } else {
        if neighbor_living_cells == 3 {
            // println!("For dead cell {} {} is ALIVE, count: {}", cell.x, cell.y, neighbor_living_cells);
            return Some(cell.clone())
        } 
        // println!("For dead cell {} {} is DEAD, count: {}", cell.x, cell.y, neighbor_living_cells);
        return None
    }
}

pub fn get_neightbor_cells(state: & State, cell: &Cell) -> Vec<Option<Cell>>{
    
    let x = cell.x as isize;
    let y = cell.y as isize;
    let frame_width = state.width as isize;
    let frame_height = state.height as isize;

    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];

    directions
        .into_iter()
        .map(|(x_offset, y_offset)| {
            let x_coord = x + x_offset;
            let y_coord = y + y_offset;
            if x_coord < 0 || x_coord > frame_width || y_coord < 0 || y_coord > frame_height{
                return None
            } else {
                return Some(Cell{x: x_coord as usize, y: y_coord as usize});
            }
        }
        ).collect()

}