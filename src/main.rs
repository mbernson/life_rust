use std::{thread, time};
use std::fmt;
use rand::prelude::*;

#[derive(Clone, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Dead => " ",
            Cell::Alive => "█",
        };
        write!(f, "{}", symbol)
    }
}

struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::Dead; width * height];
        Self {
            cells,
            width,
            height,
        }
    }

    fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height || col >= self.width {
            None
        } else {
            Some(row * self.width + col)
        }
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        let index = self.get_index(row, col)?;
        Some(&self.cells[index])
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) -> Option<()> {
        let index = self.get_index(row, col)?;
        self.cells[index] = cell;
        Some(())
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for i in (row.saturating_sub(1))..=(row + 1).min(self.height - 1) {
            for j in (col.saturating_sub(1))..=(col + 1).min(self.width - 1) {
                if i == row && j == col {
                    continue;
                }
                if let Some(Cell::Alive) = self.get_cell(i, j) {
                    count += 1;
                }
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.get_cell(row, col).unwrap();
                let count = self.count_neighbors(row, col);
                let next_cell = match (cell, count) {
                    (Cell::Alive, 2..=3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
                next_cells[row * self.width + col] = next_cell;
            }
        }
        self.cells = next_cells;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.get_cell(row, col).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let width: usize = 40;
    let height: usize = 40;
    let mut grid = Grid::new(width, height);
    let random = true;
    if random {
        for col in 0..width {
            for row in 0..height {
                let cell = if rand::random() {Cell::Alive} else {Cell::Dead};
                grid.set_cell(row, col, cell);
            }
        }
    } else {
        grid.set_cell(9, 10, Cell::Alive);
        grid.set_cell(10, 11, Cell::Alive);
        grid.set_cell(10, 12, Cell::Alive);
        grid.set_cell(9, 12, Cell::Alive);
        grid.set_cell(8, 12, Cell::Alive);
    }

    let slow: u64 = 500;
    let fast: u64 = 100;
    let sleep_duration = time::Duration::from_millis(fast);
    let mut generation = 0;
    while grid.cells.iter().filter(|&c| *c == Cell::Alive).count() > 3 {
        generation += 1;
        println!("{}", grid);  
        println!("Generation: {}", generation);
        grid.tick();
        thread::sleep(sleep_duration);
    }
}