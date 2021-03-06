#![feature(test)]
extern crate test;
mod maze;
mod search;
use rand::{thread_rng, Rng};
use test::Bencher;
 
fn test_data() -> Vec<maze::Maze> {
    let n = 1;
    let (w, h) = (400, 400);
    (0..n).map(|n| maze::Maze::random(w, h)).collect::<Vec<maze::Maze>>()
}

#[bench]
fn a_star(b: &mut Bencher) {
    let mazes = test_data();
    b.iter(|| {
        let maze = rand::thread_rng().choose(&mazes).unwrap();
        search::a_star_search(maze, (0, 0), (maze.width() - 1, maze.height() - 1));
        print!("A");
        std::io::stdout().flush();
    });
}
 
#[bench]
fn dfs(b: &mut Bencher) {
    let mazes = test_data();
    b.iter(|| {
        let maze = rand::thread_rng().choose(&mazes).unwrap();
        search::a_star_search(maze, (0, 0), (maze.width() - 1, maze.height() - 1));
        print!("D")
        std::io::stdout().flush();
    });
}
