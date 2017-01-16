use search::Graph;
use n_array::NArray;
use std::io::BufRead;
use stored::Stored;
// use std::fmt;

pub struct Level {
    // 0 = empty
    // 1 = block
    // 2 = kill
    // ... = reserved
    // 10.. = fruit k + 10
    map: NArray<u8>,
    fruits: Vec<(u8, u8)>,
    pub initial_snake: Vec<(u8, u8)>, // age => pos
    exit: (u8, u8),
}

enum Cell {
    Block,
    Kill,
    Free,
    Fruit(usize),
}

impl Level {
    fn from_chars(lines: &Vec<String>) -> Self {
        let w = lines.iter().map(|ref l| l.len()).max().unwrap();
        let h = lines.len();
        let mut level = Level {
            map: NArray::new(2, &[w, h]),
            fruits: Vec::new(),
            initial_snake: Vec::new(),
            exit: (0xFF, 0xFF),
        };
        for (line, y) in lines.iter().zip(0..h) {
            for (ch, x) in line.chars().zip(0..w) {
                let pos = (x as u8, y as u8);
                level.map[&[x, y]] = match ch {
                    ' ' => 0,
                    '#' => 1,
                    '$' => 2,
                    '@' | 'f' => {
                        let k = 10 + level.fruits.len();
                        level.fruits.push(pos);
                        k as u8
                    }
                    'X' => {
                        level.exit = pos;
                        0
                    }
                    '0'...'9' => {
                        let index = (ch as i32 - '0' as i32) as usize;
                        let path = &mut level.initial_snake;
                        if index >= path.len() {
                            path.resize(index + 1, (0xFF, 0xFF));
                        }
                        path[index] = pos;
                        0
                    }
                    _ => {
                        panic!("Unexpected char: '{}'", ch);
                    }
                }
            }
        }
        for (&(x, y), i) in level.initial_snake.iter().zip(0..) {
            if x == 0xCF || y == 0xCF {
                panic!("No position for position {} in snake!", i);
            }
        }
        level
    }
    fn char(&self, x: usize, y: usize) -> char {
        match self.map[&[x, y]] {
            0 => ' ',
            1 => '#',
            2 => '$',
            10...50 => '@',
            _ => '?',
        }
    }

    fn cell_type(&self, x: u8, y: u8, state: &State) -> Cell {
        match self.map[&[x as usize, y as usize]] {
            1 => Cell::Block,
            2 => Cell::Kill,
            0 => Cell::Free,
            f @ 10...50 => {
                if ((1 << (f - 10)) & state.fruits_left) != 0 {
                    Cell::Fruit(f as usize - 10)
                } else {
                    Cell::Free
                }
            }
            _ => panic!("Bad map"),
        }
    }
    fn enterable(&self, x: u8, y: u8, state: &State) -> bool {
        match self.cell_type(x, y, state) {
            Cell::Block | Cell::Kill => false,
            _ => true,
        }
    }

    fn supportive(&self, x: u8, y: u8, state: &State) -> bool {
        match self.cell_type(x, y, state) {
            Cell::Block | Cell::Kill | Cell::Fruit(_) => true,
            _ => false,
        }
    }

    pub fn print(&self, state: &State) {
        let dim = &self.map.magnitudes;
        for y in 0..dim[1] {
            for x in 0..dim[0] {
                let ch = if (x as u8, y as u8) == self.exit {
                    'X'
                } else if let Some(snake_index) =
                    state.snake
                        .iter()
                        .position(|&(sx, sy)| sx as usize == x && sy as usize == y) {
                    if snake_index == 0 { 'o' } else { '.' }
                } else {
                    match self.cell_type(x as u8, y as u8, state) {
                        Cell::Block => '#',
                        Cell::Kill => '$',
                        Cell::Free => ' ',
                        Cell::Fruit(f) => {
                            if state.fruits_left & (1 << f) == 0 {
                                ' '
                            } else {
                                '@'
                            }
                        }
                    }
                };

                print!("{}", ch);
            }
            println!("");
        }
        println!("");
    }
}

impl Stored for Level {
    fn load<R: BufRead>(reader: &mut R) -> Self {
        let lines: Vec<String> = reader.lines().map(|l| l.ok().unwrap()).collect();
        Level::from_chars(&lines)
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct State {
    snake: Vec<(u8, u8)>,
    fruits_left: u64,
}

impl State {
    pub fn new() -> State {
        State {
            snake: vec![],
            fruits_left: 0,
        }
    }

    pub fn head(&self, level: &Level) -> (u8, u8) {
        if self.snake.len() == 0 {
            level.exit
        } else {
            self.snake[0]
        }
    }
}

impl Graph for Level {
    type Node = State;
    type Edge = char;
    fn null_edge() -> char {
        ' '
    }
    fn start(&self) -> State {
        State {
            snake: self.initial_snake.clone(),
            fruits_left: ((1 << self.fruits.len()) - 1),
        }
    }
    fn goal(&self) -> State {
        State {
            snake: vec![],
            fruits_left: 0,
        }
    }
    fn distance(&self, a: &State, b: &State) -> usize {
        let fruit_cost = u64::count_ones(a.fruits_left ^ b.fruits_left) as usize;
        fn diff(a: u8, b: u8) -> usize {
            if a > b {
                a as usize - b as usize
            } else {
                b as usize - a as usize
            }
        }
        fn distance((ax, ay): (u8, u8), (bx, by): (u8, u8)) -> usize {
            diff(ax, bx) + diff(ay, by)
        }
        let worse = if a.fruits_left.count_ones() > b.fruits_left.count_ones() {
            &a
        } else {
            &b
        };
        // let worse_head = worse.head(self);
        // let fruit_distance = self.fruits
        // .iter()
        // .zip(0..)
        // .filter(|&(_, i)| 0 != ((1 << i) & worse.fruits_left))
        // .map(|(&f, _)| distance(worse_head, f))
        // .max()
        // .unwrap_or(0);
        //
        let distance_cost = distance(a.head(self), b.head(self));
        fruit_cost + distance_cost
    }

    fn neighbors(&self, s: &State) -> Vec<(char, State)> {
        if s.snake.len() == 0 {
            return vec![];
        }
        let &(hx, hy) = &s.snake[0];
        [('<', -1, 0), ('>', 1, 0), ('^', 0, -1), ('v', 0, 1)]
            .iter()
            .map(|&(dir, dx, dy)| (dir, (hx as i32) + dx, (hy as i32) + dy))
            .map(|(dir, nx, ny)| (dir, (nx as u8, ny as u8)))
            .filter(|&(_, (x, y))| self.enterable(x, y, s))
            .filter(|&(_, next)| !s.snake.iter().any(|&past| past == next))
            .filter_map(|(dir, next)| {
                let mut new_state = State {
                    snake: Iterator::chain([next].iter(), s.snake.iter())
                        .cloned()
                        .collect(),
                    fruits_left: s.fruits_left,
                };
                if let Cell::Fruit(f) = self.cell_type(next.0, next.1, &new_state) {
                    new_state.fruits_left &= !(1 << f);
                } else {
                    new_state.snake.pop();
                }
                if new_state.fruits_left == 0 && next == self.exit {
                    new_state.snake.clear();
                    return Some((dir, new_state));
                }
                let fall_height = (0..100)
                    .find(|fall| {
                        new_state.snake
                            .iter()
                            .any(|&(sx, sy)| self.supportive(sx, sy + fall + 1, &new_state))
                    })
                    .unwrap();
                if fall_height != 0 {
                    for &mut (_, ref mut sy) in &mut new_state.snake {
                        *sy += fall_height as u8;
                    }
                }
                let mut kill = false;
                let mut nonkill = false;
                for &(sx, sy) in &new_state.snake {
                    match self.cell_type(sx, sy + 1, &new_state) {
                        Cell::Kill => {
                            kill = true;
                        }
                        Cell::Block | Cell::Fruit(_) => {
                            nonkill = true;
                        }
                        _ => {}
                    }
                }
                if kill && !nonkill {
                    return None;
                }
                Some((dir, new_state))
            })
            .collect()
    }
}
