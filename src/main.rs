mod maze;
use std::thread;
use maze::Maze;
use std::io::stdin;

fn main() {
    let mut ig = String::new();
    stdin().read_line(&mut ig);
    let mut m = Maze::new(3, 3);
    m.right_open[2] = true;
    m.right_open[3] = true;
    m.down_open[1] = true;
    m.down_open[3] = true;
    println!("Maze:\n{}", &m);
}
