mod utils;

use js_sys::Math;
use std::fmt;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Self {
        #[cfg(feature = "wasm")]
        console::time_with_label(name);
        Self { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        #[cfg(feature = "wasm")]
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    active_cells: bool,
    cells: [Vec<Cell>; 2],
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        console_log!("Initializing universe with size: {}/{}", width, height);
        let cells: Vec<Cell> = (0..width * height).map(|_| Cell::Dead).collect();
        let cells = [cells.clone(), cells];

        Self {
            width,
            height,
            active_cells: false,
            cells,
        }
    }
    pub fn new_random(width: u32, height: u32) -> Self {
        console_log!("Initializing universe with size: {}/{}", width, height);
        let cells: Vec<Cell> = (0..width * height)
            .map(|_| {
                if Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        let cells = [cells.clone(), cells];

        Self {
            width,
            height,
            active_cells: false,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells[self.active_cells as usize].as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[self.active_cells as usize][idx].toggle();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        {
            let _timer = Timer::new("Universe::tick::new_generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[self.active_cells as usize][idx];
                    let live_neightbors = self.live_neightbor_count(row, col);

                    let next_cell = match (cell, live_neightbors) {
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        (Cell::Dead, 3) => Cell::Alive,
                        (otherwise, _) => otherwise,
                    };

                    self.cells[!self.active_cells as usize][idx] = next_cell;
                }
            }
        }

        self.active_cells = !self.active_cells;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neightbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[self.active_cells as usize][nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[self.active_cells as usize][n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[self.active_cells as usize][ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[self.active_cells as usize][w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[self.active_cells as usize][e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[self.active_cells as usize][sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[self.active_cells as usize][s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[self.active_cells as usize][se] as u8;

        count
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells[self.active_cells as usize]
    }

    pub fn set_cells_alive(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter() {
            let idx = self.get_index(*row, *col);
            self.cells[self.active_cells as usize][idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells[self.active_cells as usize]
            .as_slice()
            .chunks(self.width as usize)
        {
            for cell in line.iter() {
                let symbol = if cell == &Cell::Alive { '◼' } else { '◻' };
                write!(f, "{symbol}").unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
