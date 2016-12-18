extern crate rand;
use rand::Rng;
use search::Graph;
use std::fmt;
use std::io::BufRead;
use stored::Stored;

pub struct Maze {
    width: usize,
    height: usize,
    open: Vec<bool>,
    marked: Vec<bool>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Stored for Maze {
    fn load<R: BufRead>(reader: &mut R) -> Self {
        let lines: Vec<String> = reader.lines().map(|l| l.ok().unwrap()).collect();

        let w = lines.iter().map(|ref l| l.len()).max().unwrap();
        let h = lines.len();

        let mut maze = Maze::new(w, h);
        for (line, y) in lines.iter().zip(0..h) {
            for (ch, x) in line.chars().zip(0..w) {
                let open = match ch {
                    ' ' => true,
                    '#' => false,
                    '@' => {
                        maze.start = (x, y);
                        true
                    }
                    'X' => {
                        maze.goal = (x, y);
                        true
                    }
                    _ => {
                        panic!("Unexpected char: '{}'", ch);
                    }
                };
                if open {
                    maze.set_open(x, y);
                }
            }
        }
        maze
    }
}

impl Graph for Maze {
    type Node = (usize, usize);

    fn start(&self) -> (usize, usize) {
        self.start
    }

    fn goal(&self) -> (usize, usize) {
        self.goal
    }

    fn neighbors(&self, &(x, y): &(usize, usize)) -> Vec<(usize, usize)> {
        self.adjacents(x, y)
            .into_iter()
            .filter(|&(nx, ny)| self.is_open(nx, ny))
            .collect()
    }

    fn distance(&self, &(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)) -> usize {
        fn dist(a: usize, b: usize) -> usize {
            if a < b { b - a } else { a - b }
        }
        let d = dist(x1, x2) + dist(y1, y2);
        if d < 2 { d * 1000 } else { d * 1001 }
    }
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        assert!(width > 0 && height > 0,
                format!("mis-sized maze {}x{}", width, height));
        Maze {
            width: width,
            height: height,
            open: vec![false; width * height],
            marked: vec![false; width * height],
            start: (1, 1),
            goal: (width - 2, height - 2),
        }
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn is_open(&self, x: usize, y: usize) -> bool {
        !self.in_bounds(x, y) || self.open[y * self.width + x]
    }

    fn adjacents(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut list = Vec::<(usize, usize)>::with_capacity(4);
        // avoid pillars and borders
        if x > 0 {
            list.push((x - 1, y));
        }
        if y > 0 {
            list.push((x, y - 1));
        }
        if x < self.width - 1 {
            list.push((x + 1, y));
        }
        if y < self.height - 1 {
            list.push((x, y + 1));
        }
        list
    }

    fn set_open(&mut self, x: usize, y: usize) {
        if self.in_bounds(x, y) {
            self.open[x + y * self.width] = true;
        } else {
            panic!("Out of bounds: ({}, {})", x, y);
        }
    }

    pub fn random(width: usize, height: usize) -> Maze {
        assert!(width > 0 && height > 0 && width % 2 != 0 && height % 2 != 0,
                format!("mis-sized maze {}x{}", width, height));
        let mut maze = Self::new(width, height);
        let mut stack = Vec::new();
        stack.push((1, 1, 1, 1));
        while let Some((x, y, px, py)) = stack.pop() {
            // If we've been here or we're about to join to paths, don't
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 || maze.is_open(x, y) ||
               maze.is_open(x + x - px, y + y - py) {
                continue;
            }
            // if we're about to destroy a pillar, don't
            if x % 2 == 0 && y % 2 == 0 {
                continue;
            }
            maze.set_open(x, y);
            let mut nexts = maze.adjacents(x, y);
            rand::thread_rng().shuffle(&mut nexts);
            for (nx, ny) in nexts {
                stack.push((nx, ny, x, y));
            }
        }
        maze
    }

    pub fn mark(&mut self, x: usize, y: usize) {
        self.marked[y * self.width + x] = true;
    }
    pub fn is_marked(&self, x: usize, y: usize) -> bool {
        self.marked[y * self.width + x]
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let single = [// ┌─┬┐
                      // │ ││
                      // ├─┼┤
                      // └─┴┘
                      '│',
                      '─',
                      '─',
                      '─',
                      '│',
                      '┌',
                      '┐',
                      '┬',
                      '│',
                      '└',
                      '┘',
                      '┴',
                      '│',
                      '├',
                      '┤',
                      '┼'];
        let double = [// ╔═╦╗
                      // ║ ║║
                      // ╠═╬╣
                      // ╚═╩╝
                      '║',
                      '═',
                      '═',
                      '═',
                      '║',
                      '╔',
                      '╗',
                      '╦',
                      '║',
                      '╚',
                      '╝',
                      '╩',
                      '║',
                      '╠',
                      '╣',
                      '╬'];
        fn wall(chars: &[char; 16], left: bool, right: bool, up: bool, down: bool) -> char {
            let index = (up as usize) * 8 + (down as usize) * 4 + (left as usize) * 2 +
                        (right as usize) * 1;
            chars[index]
        }

        for y in 0..self.height {
            for x in 0..self.width {
                try!(write!(f,
                            "{}",
                            if self.is_marked(x, y) {
                                wall(&double,
                                     x <= 0 || self.is_marked(x - 1, y),
                                     self.is_marked(x + 1, y),
                                     y <= 0 || self.is_marked(x, y - 1),
                                     self.is_marked(x, y + 1))
                            } else if self.is_open(x, y) {
                                ' '
                            } else {
                                wall(&single,
                                     x > 0 && !self.is_open(x - 1, y),
                                     !self.is_open(x + 1, y),
                                     y > 0 && !self.is_open(x, y - 1),
                                     !self.is_open(x, y + 1))
                            }));
            }
            try!(f.write_str("\n"));
        }
        try!(f.write_str("\n"));
        Ok(())
    }
}
