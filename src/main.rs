extern crate getopts;
extern crate rand;
mod maze;
mod maze2;
mod search;
use getopts::{Options,HasArg,Occur};
use maze::Maze;
use maze2::Maze2;
use std::env;

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
            let mut m = Maze::random(17, 17);
            let (w, h) = m.dims();
            for (x, y) in search::a_star_search(&m, (0, 0), (w - 1, h - 1)).unwrap() {
                m.mark(x, y);
            }
            println!("Maze:\n{}", &m);
        },
        "maze2" => {
            let mut m = Maze2::random(121, 53);
            let (w, h) = m.dims();
            println!("Maze:\n{}", &m);
            for (x, y) in search::a_star_search(&m, (1, 1), (w - 2, h - 2)).unwrap() {
                m.mark(x, y);
            }
            println!("Maze:\n{}", &m);
        },
        other => {
            println!("{}\n\nUnexpected puzzle type: '{}'\n", opts.usage(brief), other);
            return;
        }
    };
}
