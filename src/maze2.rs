extern crate rand;
use search::Graph;
use rand::Rng;
use std::fmt;

pub struct Maze2 {
    width: usize,
    height: usize,
    open: Vec<bool>,
    marked: Vec<bool>,
}

impl Graph for Maze2 {
    type Node = (usize, usize);

    fn neighbors(&self, &(x, y): &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::<(usize, usize)>::with_capacity(4);

        for &(i, j) in &[(x - 1, y),
                        (x + 1, y),
                        (x, y - 1),
                        (x, y + 1)] {
            let (x, y) = (i as usize, j as usize);
            if self.is_open(x, y) {
                neighbors.push((x, y));
            }
        }
        neighbors
    }

    fn distance(&self,
                &(x1, y1): &(usize, usize),
                &(x2, y2): &(usize, usize)) -> usize {
        fn dist(a: usize, b: usize) -> usize {
            if a < b { b - a } else { a - b }
        }
        let d = dist(x1, x2) + dist(y1, y2);
        if d < 2 {
            d * 1000
        } else {
            d * 1001
        }
    }
}

impl Maze2 {
    pub fn new(width: usize, height: usize) -> Maze2 {
        Maze2 {
            width: width,
            height: height,
            open: vec![false; width * height],
            marked: vec![false; width * height],
        }
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn is_open(&self, x: usize, y: usize) -> bool {
        self.in_bounds(x, y) && self.open[y * self.height + x]
    }

    fn set_open(&mut self, x: usize, y: usize) {
        if self.in_bounds(x, y) && self.open[y * self.height + x] {
            self.open[y * self.height + x] = true;
        } else {
            panic!("Out of bounds");
        }
    }

    pub fn random(width: usize, height: usize) -> Maze2 {
        let mut maze = Self::new(width, height);
        let mut visited = vec![vec![false; width]; height];
        let mut stack = Vec::new();
        stack.push((1usize, 1usize, 1usize, 1usize));
        let mut dirs = [(0, -1), (-1,  0), (1,  0), (0,  1)];
        while let Some((x, y, mx, my)) = stack.pop() {
            if maze.is_open(x, y) {
                continue;
            }
            maze.set_open(x, y);
            maze.set_open(mx, my);
            rand::thread_rng().shuffle(&mut dirs);
            for &(dx, dy) in &dirs {
                let (mx, my) = (x + dx, y + dy);
                let (nx, ny) = (mx + dx, my + dy);
                if maze.in_bounds(nx, ny) {
                    stack.push((nx, ny, mx, my));
                }
            }
        }
        maze
    }

    pub fn mark(&mut self, x: usize, y: usize) { self.marked[y * self.width + x] = true; } 
    pub fn is_marked(&self, x: usize, y: usize) -> bool { self.marked[y * self.width + x] }

    pub fn dims(&self) -> (usize, usize) { (self.width, self.height) } 
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
}

impl fmt::Display for Maze2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_str(if self.is_marked(x, y) { "*" }
                            else if self.is_open(x, y) { " " }
                            else { "#" });
            }
        }
        Ok(())
    }
}
