use rand::prelude::SliceRandom;
use image::{RgbImage, Rgb};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>, // true for walls, false for paths
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // reverse ordering for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

    fn heuristic(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
    }

    pub fn solve(&self) -> Option<Vec<(usize, usize)>> {
        let start = (0, 0);
        let goal = (self.width - 1, self.height - 1);
        let mut open_set = BinaryHeap::new();
        open_set.push(Node { position: start, cost: 0 });

        let mut came_from = vec![vec![None; self.height]; self.width];
        let mut g_score = vec![vec![usize::MAX; self.height]; self.width];
        g_score[start.0][start.1] = 0;

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(Node { position, cost }) = open_set.pop() {
            if position == goal {
                let mut path = vec![position];
                let mut current = position;
                while let Some(prev) = came_from[current.0][current.1] {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                return Some(path);
            }

            for &(dx, dy) in &directions {
                let neighbor = ((position.0 as isize + dx) as usize, (position.1 as isize + dy) as usize);
                if neighbor.0 < self.width && neighbor.1 < self.height && !self.cells[neighbor.0][neighbor.1] {
                    let tentative_g_score = g_score[position.0][position.1] + 1;
                    if tentative_g_score < g_score[neighbor.0][neighbor.1] {
                        came_from[neighbor.0][neighbor.1] = Some(position);
                        g_score[neighbor.0][neighbor.1] = tentative_g_score;
                        let f_score = tentative_g_score + self.heuristic(neighbor, goal);
                        open_set.push(Node { position: neighbor, cost: f_score });
                    }
                }
            }
        }

        None
    }

    pub fn save_as_png(&self, path: &str, solution: Option<&[(usize, usize)]>) {
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

        if let Some(solution) = solution {
            for &(x, y) in solution {
                img.put_pixel(x as u32, y as u32, Rgb([255, 0, 0])); // Red for solution path
            }
        }

        img.save(path).unwrap();
    }
}
