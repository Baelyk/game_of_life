extern crate piston_window;

use std::fmt;
use piston_window::*;
use piston_window::grid::Grid;

const CELL_SIZE: u32 = 5;
const GRID_SIZE: u32 = 100;
const GRID_RADIUS: f64 = CELL_SIZE as f64 / 20.0; // The radius of the grid lines
const CELL_OFFSET: f64 = CELL_SIZE as f64 / 40.0; // The offset of the cell from the grid lines

#[derive(Copy, Clone)]
struct Cell {
    alive: bool,
    x: u32,
    y: u32
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut alive = "Dead";
        if self.alive {
            alive = "Live";
        }
        write!(f, "{} Cell at ({},{})", alive, self.x, self.y)
    }
}

impl Cell {
    fn neighbours (&self) -> [(u32, u32); 8] { // TODO: There has to be a better way to do this
        let mut neighbours = [(self.x, self.y); 8];
        const POSITIONS: [(i32, i32); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

        for i in 0..8 {
            let (x, y) = POSITIONS[i];
            if self.x as i32 + x >= 0
                && self.y as i32 + y >= 0
                && self.x as i32 + x < GRID_SIZE as i32
                && self.y as i32 + y < GRID_SIZE as i32 {
                neighbours[i] = ((self.x as i32 + x) as u32, (self.y as i32 + y) as u32);
            }
        }

        return neighbours
    }
    fn toggle_alive (&mut self) {
        if self.alive {
            self.alive = false;
        } else {
            self.alive = true;
        }
    }
}

fn game (cells: [[Cell; GRID_SIZE as usize]; GRID_SIZE as usize]) -> [[Cell; GRID_SIZE as usize]; GRID_SIZE as usize] {
    /*
    1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    2. Any live cell with two or three live neighbours lives on to the next generation.
    3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    */
    let mut new_cells = cells;
    for x in 0..GRID_SIZE as usize {
        for y in 0..GRID_SIZE as usize {
            let mut cell = cells[x][y]; // So the changes aren't taking into account until the next generation
            let mut living_neighbours = 0;
            // println!("{:?}", cell.neighbours());
            for &(nx, ny) in cell.neighbours().iter() {
                // println!("!({} == {} as u32 = {} && {}) = {} && {}: {}", nx, x, nx == x as u32, ny == y as u32, !(nx == x as u32 && ny == y as u32), cells[nx as usize][ny as usize].alive, !(nx == x as u32 && ny == y as u32) && cells[nx as usize][ny as usize].alive);
                if !(nx == x as u32 && ny == y as u32) && cells[nx as usize][ny as usize].alive  {
                    living_neighbours += 1;
                }
            }
            // println!("{},{}: {}", x, y, living_neighbours);
            if cell.alive {
                // println!("Live {:?}", cell);
                if living_neighbours < 2 { // Underpopulation
                    // println!("Under {:?}", cell);
                    cell.alive = false;
                } else if living_neighbours > 3 { // Overpopulation
                    // println!("Over {:?}", cell);
                    cell.alive = false;
                }
            } else {
                // println!("Dead {:?}", cell);
                if living_neighbours == 3 { // Reproduction
                    // println!("Repro {:?}", cell);
                    cell.alive = true;
                }
            }
            new_cells[x][y] = cell;
        }
    }

    new_cells
}

fn populate_cells () -> [[Cell; GRID_SIZE as usize]; GRID_SIZE as usize] {
    let mut array = [[Cell { alive: false, x: 0, y: 0 }; GRID_SIZE as usize]; GRID_SIZE as usize];
    for x in 0..GRID_SIZE as usize {
        for y in 0..GRID_SIZE as usize {
            array[x][y] = Cell {
                alive: false,
                x: x as u32,
                y: y as u32
            };
        }
    }
    array
}

fn main() {
    let mut cells = populate_cells();

    let mut window: PistonWindow = WindowSettings::new("Game of Life", [GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE]).exit_on_esc(true).resizable(false).build().unwrap();

    let grid = Grid {
        cols: GRID_SIZE,
        rows: GRID_SIZE,
        units: CELL_SIZE as f64
    };
    let mut mouse: (u32, u32) = (0, 0);
    let mut update = true;
    let mut ups = 1;
    let mut focused = false;

    window.events.set_ups(ups);
    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear([1.0; 4], g);
            grid.draw(&Line::new([0.5, 0.5, 0.5, 1.0], GRID_RADIUS), &DrawState {
                scissor: None,
                stencil: None,
                blend: None
            }, c.transform, g);
            for x in 0..GRID_SIZE as usize {
                for y in 0..GRID_SIZE as usize {
                    if cells[x][y].alive {
                        rectangle([0.0, 0.0, 0.0, 1.0], [(x * CELL_SIZE as usize + CELL_OFFSET as usize) as f64, (y * CELL_SIZE as usize + CELL_OFFSET as usize) as f64, CELL_SIZE as f64 - CELL_OFFSET, CELL_SIZE as f64 - CELL_OFFSET], c.transform, g);
                    }
                }
            }
        });
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            if focused {
                let (x, y) = mouse;
                let cell_x = (x - x % CELL_SIZE) / CELL_SIZE;
                let cell_y = (y - y % CELL_SIZE) / CELL_SIZE;
                cells[cell_x as usize][cell_y as usize].toggle_alive();
                // println!("{:?}", cells[cell_x as usize][cell_y as usize]);
            }
        }
        if let Some(Button::Keyboard(Key::Space)) = event.press_args() {
            if update {
                update = false;
            } else {
                update = true;
            }
        }
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            ups += 1;
            window.events.set_ups(ups);
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            if ups > 1 { ups -= 1; }
            window.events.set_ups(ups);
        }
        event.update(|time| {
            if update {
                window.draw_2d(&event, |c, g| {
                    clear([1.0; 4], g);
                    grid.draw(&Line::new([0.5, 0.5, 0.5, 1.0], GRID_RADIUS), &DrawState {
                        scissor: None,
                        stencil: None,
                        blend: None
                    }, c.transform, g);
                    for x in 0..GRID_SIZE as usize {
                        for y in 0..GRID_SIZE as usize {
                            if cells[x][y].alive {
                                rectangle([0.0, 0.0, 0.0, 1.0], [(x * CELL_SIZE as usize + CELL_OFFSET as usize) as f64, (y * CELL_SIZE as usize + CELL_OFFSET as usize) as f64, CELL_SIZE as f64 - CELL_OFFSET, CELL_SIZE as f64 - CELL_OFFSET], c.transform, g);
                            }
                        }
                    }
                });
                cells = game(cells);
            }
        });
        event.mouse_cursor(|x, y| {
            if x > 0.0 && x < (GRID_SIZE * CELL_SIZE) as f64 && y > 0.0 && x < (GRID_SIZE * CELL_SIZE) as f64 {
                focused = true;
                mouse = (x as u32, y as u32);
            } else {
                focused = false;
            }
        });
    }
}
