extern crate rand;
mod maze;
mod search;

fn main() {
    let m = maze::Maze::random(25, 25);
    search::dfs_search(&m, maze::Position::new(0, 0), |&m, &p| 3.2);
    println!("Maze:\n{}", &m);

}
