extern crate getopts;
extern crate rand;
mod maze;
mod search;
mod stored;
use getopts::{Options,HasArg,Occur};
use maze::Maze;
use std::env;
use std::fs::File;
use std::io::BufReader;
use stored::Stored;

/*
fn main() {
    let (w, h) = (10, 10);
    let mut m = maze::Maze::random(w, h);
    for (x, y) in search::a_star_search(&m, (0, 0), (w - 1, h - 1)).unwrap() {
        m.mark(x, y);
    }
    println!("Maze:\n{}", &m);
}
*/

fn main() {
    let mut opts = Options::new();
    let brief = "Solves puzzles.";

    opts.opt("i", "input", "Input puzzle.",
             "INPUT", HasArg::Yes, Occur::Optional);
    opts.opt("t", "type", "Puzzle type. Either 'snake' or 'maze'.",
             "TYPE", HasArg::Yes, Occur::Req);
    let m = match opts.parse(env::args()) {
        Ok(m) => { m }
        Err(f) => {
            println!("{}\n\n{}", opts.usage(brief), f);
            return;
        }
    };
    let input = m.opt_str("input");
    match m.opt_str("type").unwrap().as_ref() {
        "snake" => {
        },
        "maze" => {
            let dims = (201, 201);
            let mut m = if let Some(file) = input {
                    Maze::load(&mut BufReader::new(&mut File::open(file).ok().unwrap()))
                } else {
                    Maze::random(dims.0, dims.1)
                };
            let (w, h) = m.dims();
            println!("Maze:\n{}", &m);
            if let Some(solution) = search::a_star_search(&m, (1, 1), (w - 2, h - 2)) {
                for (x, y) in solution {
                    m.mark(x, y);
                }
                println!("Maze:\n{}", &m);
            } else {
                println!("No solution!\n");
            }
        },
        other => {
            println!("{}\n\nUnexpected puzzle type: '{}'\n", opts.usage(brief), other);
            return;
        }
    };
}
