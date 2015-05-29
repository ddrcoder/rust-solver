use std::fmt;

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
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fence = |low, high, end, gap, post| ->
            end();
            for v in low..high - 1 {
                gap(v);
                post(v);
            }
            gap(high - 1);
            end();
        };
        let rwall = |f: &mut fmt::Formatter, x, y| -> () {
            let r = self.right_open[x + y * (self.width - 1)];
            write!(f, "{}", if r { ' ' } else { '|' });
        };
        write!(f, "+");
        for x in 0..self.width {
            write!(f, "-+");
        }
        writeln!(f, "");

        for y in 0..self.height - 1 {
            write!(f, "|");
            for x in 0..self.width - 1 {
                write!(f, " ");
                rwall(f, x, y);
            }
            write!(f, " ");
            writeln!(f, "|");
            write!(f, "+");
            for x in 0..self.width {
                let d = self.down_open[x + y * self.width];
                write!(f, "{}", if d { ' ' } else { '-' });
                write!(f, "+");
            }
            writeln!(f, "");
        }
        write!(f, "|");
        for x in 0..self.width - 1 {
            write!(f, " ");
            rwall(f, x, self.height - 1);
        }
        write!(f, " ");
        writeln!(f, "|");
        write!(f, "+");
        for x in 0..self.width {
            write!(f, "-+");
        }
        writeln!(f, "");
        Ok(())
    }
}

