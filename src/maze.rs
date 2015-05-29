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
        Maze {
            width: width,
            height: height,
            right_open: vec![false; (width - 1) * height],
            down_open: vec![false; width * (height - 1)],
        }
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

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rwall = |x, y| if self.right(x, y) { " " } else { "|" };
        let dwall = |x, y| if self.down(x, y) { " " } else { "-" };
        let (w, h) = (self.width, self.height);
        f.write_str(&fence(0, h,
                           &fence(0, w, "+", |_| "-", |_| "+"),
                           |y| &fence(0, w, "|", |_| " ", |x| rwall(x, y)),
                           |y| &fence(0, w, "+", |x| dwall(x, y), |_| "+")))
    }
}

