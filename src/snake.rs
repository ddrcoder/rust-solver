use search::Graph;
use n_array::NArray;
use std::io::BufRead;
use stored::Stored;
use std::fmt;

pub struct Level {
    blocks: NArray<bool>,
    fruits: Vec<(u8, u8)>,
    initial_snake: Vec<(u8, u8)>, // age => pos
    exit: (u8, u8),
}

impl Level {
    fn from_chars(lines: &Vec<String>) -> Self {
        let w = lines.iter().map(|ref l| l.len()).max().unwrap();
        let h = lines.len();
        let mut level = Level {
            blocks: NArray::new(2, &[w, h]),
            fruits: Vec::new(),
            initial_snake: Vec::new(),
            exit: (0xFF, 0xFF),
        };
        for (line, y) in lines.iter().zip(0..h) {
            for (ch, x) in line.chars().zip(0..w) {
                let pos = (x as u8, y as u8);
                match ch {
                    '#' => {
                        level.blocks[&[x, y]] = true;
                    }
                    ' ' => {}
                    'f' => {
                        level.fruits.push(pos);
                    }
                    'X' => {
                        level.exit = pos;
                    }
                    '0'...'9' => {
                        let index = (ch as i32 - '0' as i32) as usize;
                        let path = &mut level.initial_snake;
                        if index >= path.len() {
                            path.resize(index + 1, (0xCF, 0xCF));
                        }
                        path[index] = pos;
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
}

impl Stored for Level {
    fn load<R: BufRead>(reader: &mut R) -> Self {
        let lines: Vec<String> = reader.lines().map(|l| l.ok().unwrap()).collect();
        Level::from_chars(&lines)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dim = &self.blocks.magnitudes;
        for y in 0..dim[1] {
            for x in 0..dim[0] {
                write!(f, "{}", if self.blocks[&[x, y]] { '#' } else { ' ' })?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct State {
    snake: Vec<(u8, u8)>,
    fruits_left: u64,
}

impl State {
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
        let worse_head = worse.head(self);
        let fruit_distance = self.fruits
            .iter()
            .zip(0..)
            .filter(|&(_, i)| 0 != ((1 << i) & worse.fruits_left))
            .map(|(&f, _)| distance(worse_head, f))
            .max()
            .unwrap_or(0);
        let distance_cost = distance(a.head(self), b.head(self));
        fruit_cost + fruit_distance + distance_cost
    }

    fn neighbors(&self, s: &State) -> Vec<(char, State)> {
        if s.snake.len() == 0 {
            return vec![];
        }
        let &(hx, hy) = &s.snake[0];
        [('<', -1, 0), ('>', 1, 0), ('^', 0, -1), ('v', 0, 1)]
            .iter()
            .map(|&(dir, dx, dy)| (dir, (hx as i32) + dx, (hy as i32) + dy))
            .filter(|&(_, x, y)| !self.blocks[&[x as usize, y as usize]])
            .map(|(dir, nx, ny)| (dir, (nx as u8, ny as u8)))
            .filter(|&(_, next)| !s.snake.iter().any(|&past| past == next))
            .map(|(dir, next)| {
                let fruit_hit = self.fruits
                    .iter()
                    .position(|&fruit_pos| fruit_pos == next);
                let new_fruits_left = if let Some(index) = fruit_hit {
                    s.fruits_left & !(1 << index)
                } else {
                    s.fruits_left
                };
                let new_snake = if new_fruits_left == 0 && next == self.exit {
                    vec![]
                } else {
                    let new_length = if s.fruits_left == new_fruits_left {
                        s.snake.len()
                    } else {
                        s.snake.len() + 1
                    };
                    let mut falling: Vec<(u8, u8)> = Iterator::chain([next].iter(), s.snake.iter())
                        .take(new_length)
                        .cloned()
                        .collect();
                    let fall_height = (0..100)
                        .find(|fall| {
                            falling.iter().any(|&(sx, sy)| {
                                self.blocks[&[sx as usize, sy as usize + 1 + fall]]
                            })
                        })
                        .unwrap();
                    if fall_height != 0 {
                        for &mut (_, ref mut sy) in &mut falling {
                            *sy += fall_height as u8;
                        }
                        println!("Falling {}", fall_height);
                    }
                    falling
                };
                (dir,
                 State {
                    snake: new_snake,
                    fruits_left: new_fruits_left,
                })
            })
            .collect()
    }
}
