use rand::prelude::SliceRandom;
use image::{RgbImage, Rgb};

pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>, // true for walls, false for paths
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![true; height]; width];
        Maze { width, height, cells }
    }

    pub fn generate(&mut self) {
        let mut stack = vec![(0, 0)];
        self.cells[0][0] = false;

        let directions = [(2, 0), (-2, 0), (0, 2), (0, -2)];
        let mut rng = rand::thread_rng();

        while let Some((x, y)) = stack.pop() {
            let mut neighbors = vec![];
            for &(dx, dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height && self.cells[nx as usize][ny as usize] {
                    neighbors.push((nx as usize, ny as usize));
                }
            }

            if !neighbors.is_empty() {
                stack.push((x, y));
                let &(nx, ny) = neighbors.choose(&mut rng).unwrap();
                self.cells[nx][ny] = false;
                self.cells[(x + nx) / 2][(y + ny) / 2] = false;
                stack.push((nx, ny));
            }
        }

        // Ensure there is a path from left to right
        for i in 0..self.height {
            self.cells[0][i] = false;
            self.cells[self.width - 1][i] = false;
        }
    }

    pub fn save_as_png(&self, path: &str) {
        let mut img = RgbImage::new(self.width as u32, self.height as u32);

        for x in 0..self.width {
            for y in 0..self.height {
                let color = if self.cells[x][y] {
                    Rgb([0, 0, 0]) // Black for walls
                } else {
                    Rgb([255, 255, 255]) // White for paths
                };
                img.put_pixel(x as u32, y as u32, color);
            }
        }

        img.save(path).unwrap();
    }
}
