use search::Graph;
use n_array::NArray;
use std::io::BufRead;
use stored::Stored;
use std::fmt;

struct SnakeState {
    path: Vec<(u8, u8)>,
}

pub struct Level {
    blocks: NArray<bool>,
    fruits: Vec<(u8, u8)>,
    initial_snake: SnakeState,
}

impl Level {
    fn from_chars(lines: &Vec<String>) -> Self {
        let w = lines.iter().map(|ref l| l.len()).max().unwrap();
        let h = lines.len();
        let mut level = Level {
            blocks: NArray::new(2, &[w, h]),
            fruits: Vec::new(),
            initial_snake: SnakeState { path: Vec::new() },
        };
        for (line, y) in lines.iter().zip(0..h) {
            for (ch, x) in line.chars().zip(0..w) {
                match ch {
                    '#' => {
                        level.blocks[&[x, y]] = true;
                    },
                    ' ' => {}
                    'f' => {
                        level.fruits.push((x as u8, y as u8));
                    }
                    '0'...'9' => {
                        let index = ch as i32 as usize;
                        let path = &mut level.initial_snake.path;
                        if index >= path.len() {
                            path.resize(index + 1, (0xFF, 0xFF));
                        }
                    }
                    _ => {
                        panic!("Unexpected char: '{}'", ch);
                    }
                }
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
                write!(f, "{}", if self.blocks[&[x, y]] { '#' } else { ' ' });
            }
        }
        Ok(())
    }
}
