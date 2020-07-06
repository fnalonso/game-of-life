
use rand;
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

use crate::terminal::Color;
use crate::terminal;


struct Cell {
    x: i32,
    y: i32,
    alive: bool,
    alive_next_generation: bool,
    neighbours: Vec<Rc<RefCell<Cell>>>
}

impl Cell {

    // (x, y) position from the center
    const NEIGHBOUR_POSITIONS:[(i32, i32);8]= [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    pub fn new(x: i32, y: i32, alive:bool) -> Cell {
        Cell { x, y, alive, alive_next_generation:false, neighbours: Vec::new() }
    }


    pub fn add_neighbour(&mut self, neighbour:&Rc<RefCell<Cell>>) {
        self.neighbours.push(Rc::clone(&neighbour))
    }

    pub fn draw(&self, padding: i32){
        let mut color = Color::Grey;
        if self.alive == true {
            color = Color::Black
        }
        terminal::print_cell(self.x  + padding, self.y * 2 + padding, color);

    }

}


pub struct Cells {
    cells: Vec<Vec<Rc<RefCell<Cell>>>>,
 }

impl Cells {
    /// Create a random population based on received parameters
    /// The `population_percentage` defines the percentage that will start alive
    pub fn create_from_random(rows: i32, columns: i32, population_percentage: f32) -> Cells {

        let mut randomizer = rand::thread_rng();
        let mut cells = Vec::new();
            for i in 0..rows {
                let mut cell_columns = Vec::new();
                for j in 0..columns {

                    let v = randomizer.gen_range(1, 100) as f32;
                    let alive = if v < population_percentage {
                        true
                    } else {
                        false
                    };
                    let cell = Cell::new(i, j, alive);
                    cell_columns.push(Rc::new(RefCell::new(cell)))
                }
            cells.push(cell_columns);
        }
        Cells { cells }
    }

    /// Create the population based on a layout file.
    /// Check the data/ folder to see how to add a custom layout.
    ///
    /// # Panic
    ///
    /// If any row(line) has the size different from the first line, a panic will occurs
    ///
    pub fn create_from_layout(layout_data:String) -> Cells {
        let mut cells = Vec::new();
        let rows:Vec<&str> = layout_data.split('\n').collect();
        let rows_size = rows[0].len();

        for (i, &row) in rows.iter().enumerate() {
            assert_eq!(row.len(), rows_size);
            let columns = row.as_bytes();
            let mut cell_columns = Vec::new();
            let mut x_position = 0;
            for cell in columns {
                let alive:bool;
                if cell == &84_u8 {
                    alive = true;
                } else {
                    alive = false;
                }
                let cell = Cell::new(i as i32, x_position, alive);
                cell_columns.push(Rc::new(RefCell::new(cell)));
                x_position += 1;
            }
            cells.push(cell_columns);
        }
        Cells { cells }
    }

    /// Print the current state from each cell in the population
    pub fn print_cells_state(&self) {
        for row in &self.cells {
            for cell in row {
                let c = cell.borrow();
                if c.alive == true {
                    print!("T,");
                } else {
                    print!("F,");
                }
            }
            println!();
        }
    }

    /// Sets the neighbours for each cell in the population.
    pub fn populate_neighbours(&self) {
        for row in &self.cells {
            for cell in row {
                let mut t = cell.borrow_mut();
                for position in Cell::NEIGHBOUR_POSITIONS.iter() {
                    match self.cells.get(( t.x + position.0) as usize) {
                        Some(r) => {
                            match r.get((t.y + position.1) as usize) {
                                Some(c) => {
                                    t.add_neighbour(c)
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Iterate over all cells and check if they state for the next round
    /// The algorithm uses two loops to do this task.
    /// 1 - Check the state for the next generation without change the cell `alive` attribute
    /// 2 - Update the `alive` state attribute
    ///
    ///
    pub fn update_population(&self) {
        for row in &self.cells {
            for cell in row {
                let mut total_alive = 0;
                for neighbour in &cell.borrow().neighbours {
                    if neighbour.borrow().alive == true {
                        total_alive += 1;
                    }
                }

                let current_state = cell.borrow().alive;
                let mut alive:bool = false;

                if current_state == true {
                    if total_alive == 2 || total_alive == 3 {
                        alive = true;
                    }
                } else {
                    if total_alive == 3 {
                        alive = true
                    }
                }

                cell.borrow_mut().alive_next_generation = alive;

            }
        }

        for row in &self.cells {
            for cell in row {
                let alive_next = cell.borrow().alive_next_generation;
                cell.borrow_mut().alive = alive_next;

            }
        }
    }

    pub fn draw_cells(&self, padding: i32) {
        for cell_row in &self.cells {
            for cell in cell_row {
                cell.borrow().draw(padding)
            }
        }
    }

}
