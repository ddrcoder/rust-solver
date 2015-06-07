extern crate rand;
use search::Graph;
use rand::Rng;
use std::fmt;

pub struct Maze {
    width: usize,
    height: usize,
    // (i, j) is open to (i + 1, j) iff rightOpen[i + (width - 1) * j]
    // (width - 1, j) is never open to out-of-bounds
    right_open: Vec<bool>,
    // (i, j) is open to (i, j + 1) iff downOpen[i + width * j]
    // (i, height - 1) is never open to out-of-bounds
    down_open: Vec<bool>,
    marked: Vec<bool>,
}

impl Graph for Maze {
    type Node = (usize, usize);
    fn neighbors(&self, &(x, y): &Self::Node) -> Vec<Self::Node> {
        let mut neighbors = Vec::<Self::Node>::with_capacity(4);
        if x > 0 && self.left(x, y) { neighbors.push((x - 1, y)) }
        if y > 0 && self.up(x, y) { neighbors.push((x, y - 1)) }
        if x + 1 < self.width && self.right(x, y) { neighbors.push((x + 1, y)) }
        if y + 1 < self.height && self.down(x, y) { neighbors.push((x, y + 1)) }
        neighbors
    }
    fn distance(&self, &(x1, y1): &Self::Node, &(x2, y2): &Self::Node) -> usize {
        fn dist(a: usize, b: usize) -> usize {
            if a < b { b - a } else { a - b }
        }
        dist(x1, x2) + dist(y1, y2)
    }
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        Maze {
            width: width,
            height: height,
            right_open: vec![false; (width - 1) * height],
            down_open: vec![false; width * (height - 1)],
            marked: vec![false; width * height],
        }
    }

    pub fn mark(&mut self, x: usize, y: usize) {
        self.marked[y * self.width + x] = true;
    }

    pub fn is_marked(&self, x: usize, y: usize) -> bool {
        self.marked[y * self.width + x]
    }

    pub fn random(width: usize, height: usize) -> Maze{
        let mut maze = Self::new(width, height);
        let mut visited = vec![vec![false; width]; height];
        let mut stack = Vec::new();
        stack.push((0i64, 0i64, 0i64, 0i64));
        let mut nexts = [(0, -1), (-1,  0), (1,  0), (0,  1)];
        while let Some((x, y, px, py)) = stack.pop() {
            let visited = &mut visited[x as usize][y as usize];
            if *visited {
                continue
            }
            *visited = true;
            if px != x || py != y {
                let (dx, dy) = (x - px, y - py);
                let (px, py) = (px as usize, py as usize);
                match (dx, dy) {
                    (-1, 0) => maze.right_open[px - 1 + py * (width - 1)] = true,
                    (1, 0) => maze.right_open[px + py * (width - 1)] = true,
                    (0, -1) => maze.down_open[px + (py - 1) * width] = true,
                    (0, 1) => maze.down_open[px + py * width] = true,
                    (_, _) => panic!(),
                }
            }
            rand::thread_rng().shuffle(&mut nexts);
            for &(dx, dy) in &nexts {
                let (nx, ny) = (x + dx, y + dy);
                if (nx as usize) < width && (ny as usize) < height {
                    stack.push((nx, ny, x, y));
                }
            }
        }
        maze
    }

    pub fn left(&self, x: usize, y: usize) -> bool {
        return x > 0 && self.right_open[x - 1 + y * (self.width - 1)]
    }

    pub fn down(&self, x: usize, y: usize) -> bool {
        return y < self.height - 1 && self.down_open[x + y * self.width]
    }

    pub fn up(&self, x: usize, y: usize) -> bool {
        return y > 0 && self.down_open[x + (y - 1) * self.width]
    }

    pub fn right(&self, x: usize, y: usize) -> bool {
        return x < self.width - 1 && self.right_open[x + y * (self.width - 1)]
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fence<'a, 'b, Gap: Fn(usize) -> &'a str, Post : Fn(usize) -> &'b str>(
                low: usize, high: usize, end: &str, gap: Gap, post: Post) -> String {
            let mut ret = String::new();
            ret.push_str(&end);
            for v in low..high {
                ret.push_str(&gap(v));
                if v != high - 1 {
                    ret.push_str(&post(v));
                }
            }
            ret.push_str(&end);
            ret.push_str("\n");
            ret
        }

        let rwall = |x, y| if self.right(x, y) { " " } else { "|" };
        let dwall = |x, y| if self.down(x, y) { "  " } else { "--" };
        let mark = |x, y| if self.is_marked(x, y) { "**" } else { "  " };
        let (w, h) = (self.width, self.height);

        f.write_str(&fence(0, h,
                           &fence(0, w, "+", |_| "--", |_| "+"),
                           |y| &fence(0, w, "|", |x| mark(x, y), |x| rwall(x, y)),
                           |y| &fence(0, w, "+", |x| dwall(x, y), |_| "+")))
    }
}
