use std::fmt;
use std::ops::{Sub};

pub struct Maze {
    width: usize,
    height: usize,
    // (i, j) is open to (i + 1, j) iff rightOpen[i + (width - 1) * j]
    // (width - 1, j) is never open to out-of-bounds
    pub right_open: Vec<bool>,
    // (i, j) is open to (i, j + 1) iff downOpen[i + width * j]
    // (i, height - 1) is never open to out-of-bounds
    pub down_open: Vec<bool>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let a = [false; 10];
        Maze {
            width: width,
            height: height,
            right_open: vec![false; (width - 1) * height],
            down_open: vec![false; width * (height - 1)],
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fence<End, Gap, Post>(f: &mut fmt::Formatter,
                                 low: usize,
                                 high: usize,
                                 end: End,
                                 gap: Gap,
                                 post: Post) -> fmt::Result
            where End : Fn(&mut fmt::Formatter) -> fmt::Result,
                  Gap : Fn(&mut fmt::Formatter, usize) -> fmt::Result ,
                 Post : Fn(&mut fmt::Formatter, usize) -> fmt::Result {
            end(f);
            for v in low..high - 1 {
                gap(f, v);
                post(f, v);
            }
            gap(f, high - 1);
            end(f);
            writeln!(f, "");
            Ok(())
        };
        fence(f, 0, self.height,
              |f| fence(f, 0, self.width,
                        |f| write!(f, "+"),
                        |f,_| write!(f, " "),
                        |f,_| write!(f, "+")),
              |f, y| fence(f, 0, self.width,
                           |f| write!(f, "|"),
                           |f, x| write!(f, " "),
                           |f, x| {
                               let r = self.right_open[x + y * (self.width - 1)];
                               write!(f, "{}", if r { ' ' } else { '|' })
                           }),
              |f, y| fence(f, 0, self.width,
                           |f| write!(f, "+"),
                           |f, x| {
                               let r = self.down_open[x + y * self.width];
                               write!(f, "{}", if r { ' ' } else { '-' })
                           },
                           |f, x| write!(f, "+")));
        writeln!(f, "");
        Ok(())
    }
}

