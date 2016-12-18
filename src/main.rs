extern crate getopts;
extern crate n_array;
extern crate rand;
mod maze;
mod snake;
mod search;
mod stored;
use getopts::{Options, HasArg, Occur};
use maze::Maze;
use std::env;
use std::fs::File;
use std::io::BufReader;
use stored::Stored;

// fn main() {
// let (w, h) = (10, 10);
// let mut m = maze::Maze::random(w, h);
// for (x, y) in search::a_star_search(&m, (0, 0), (w - 1, h - 1)).unwrap() {
// m.mark(x, y);
// }
// println!("Maze:\n{}", &m);
// }
//
fn maze(input: Option<String>) {
    let mut m = if let Some(file) = input {
        Maze::load(&mut BufReader::new(&mut File::open(file).ok().unwrap()))
    } else {
        Maze::random(40, 40)
    };
    let (w, h) = m.dims();
    if let Some(solution) = search::a_star_search(&m) {
        for (x, y) in solution {
            m.mark(x, y);
        }
        println!("Maze:\n{}", &m);
    } else {
        println!("No solution!\n");
    }
}

fn snake(input: Option<String>) {
    let mut m = if let Some(file) = input {
        snake::Level::load(&mut BufReader::new(&mut File::open(file).ok().unwrap()))
    } else {
        panic!("Snakes need levels");
    };
    println!("Level:\n{}", &m);
    if let Some(solution) = search::a_star_search(&m) {
        let mut i = 0;
        for (a, b) in solution.iter().zip(solution.iter().skip(1)) {
            let ((ax, ay), (bx, by)) = (a.head(&m), b.head(&m));
            let direction = if ax != bx {
                if bx < ax { '<' } else { '>' }
            } else {
                if by < ay { '^' } else { 'v' }
            };
            print!("{}", direction);
            i += 1;
            if i % 4 == 0 {
                println!("");
            }
        }
        println!("Maze:\n{}", &m);
    } else {
        println!("No solution!\n");
    }
    //
}

fn main() {
    let mut opts = Options::new();
    let brief = "Solves puzzles.";

    opts.opt("i",
             "input",
             "Input puzzle.",
             "INPUT",
             HasArg::Yes,
             Occur::Optional);
    opts.opt("t",
             "type",
             "Puzzle type. Either 'snake' or 'maze'.",
             "TYPE",
             HasArg::Yes,
             Occur::Optional);
    let m = match opts.parse(env::args()) {
        Ok(m) => m,
        Err(f) => {
            println!("{}\n\n{}", opts.usage(brief), f);
            return;
        }
    };
    let input = m.opt_str("input");
    match m.opt_str("type") {
        Some(ref arg) => {
            match &arg[..] {
                // "snake" => {        },
                "maze" => {
                    maze(input);
                }
                "snake" => {
                    snake(input);
                }
                other => {
                    println!("{}\n\nUnexpected puzzle type: '{}'\n",
                             opts.usage(brief),
                             other);
                    return;
                }
            }
        }
        None => {
            maze(None);
        }
    };
}
