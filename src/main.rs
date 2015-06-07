extern crate rand;
mod maze;
mod search;

fn main() {
    let (w, h) = (25, 25);
    let mut m = maze::Maze::random(w, h);
    for (x, y) in search::a_star_search(&m, (0, 0), (w - 1, h - 1)) {
        m.mark(x, y);
    }
    println!("Maze:\n{}", &m);
}
