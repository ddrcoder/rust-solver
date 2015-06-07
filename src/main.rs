extern crate rand;
mod maze;
mod search;

fn main() {
    let mut m = maze::Maze::random(25, 25);
    for (x, y) in search::a_star_search(&m, (0, 0), (24, 24)) {
        m.mark(x, y);
    }
    println!("Maze:\n{}", &m);

}
