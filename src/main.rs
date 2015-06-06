extern crate rand;
mod maze;
mod search;

fn main() {
    let m = maze::Maze::random(25, 25);
    search::a_star_search(&m, (0, 0), (20, 20));
    println!("Maze:\n{}", &m);

}
