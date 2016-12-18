extern crate getopts;
extern crate n_array;
extern crate rand;
mod maze;
mod snake;
mod search;
use search::Graph;
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

enum Strategy {
    DFS,
    AStar,
}

fn solve<G: Graph>(graph: &G, strat: Strategy) -> Option<Vec<(G::Edge,G::Node)>> {
    match strat {
        Strategy::DFS => search::dfs_search(graph),
        Strategy::AStar => search::a_star_search(graph),
    }
}
fn maze(input: Option<String>, strat: Strategy) {
    let mut m = if let Some(file) = input {
        Maze::load(&mut BufReader::new(&mut File::open(file).expect("Couldn't open file")))
    } else {
        Maze::random(40, 40)
    };
    let result = solve(&m, strat);
    if let Some(solution) = result {
        for (_,(x, y)) in solution {
            m.mark(x, y);
        }
        println!("Maze:\n{}", &m);
    } else {
        println!("No solution!\n");
    }
}

fn snake(input: Option<String>, strat: Strategy) {
    let m =
        snake::Level::load(&mut BufReader::new(
                &mut File::open(input.expect("Snakebird requires input files"))
                .expect("Couldn't open file")));
    let result = solve(&m, strat);
    if let Some(solution) = result {
        let mut i = 0;
        for (edge, _) in solution {
            print!("{}", edge);
            i += 1;
            if i % 4 == 0 {
                println!("");
            }
        }
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
    opts.opt("s",
             "strategy",
             "Strategy. Either 'dfs' or 'astar'.",
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
    let strat = match m.opt_str("strategy").as_ref().map(|s| s.as_str()).unwrap_or("dfs") {
        "dfs" => Strategy::DFS,
        "a_star" => Strategy::AStar,
        other => panic!("Unexpected strategy: {}", other),
    };
    let input = m.opt_str("input");
    match m.opt_str("type").as_ref().map(|s| s.as_str()).unwrap_or("maze") {
        "maze" => maze(input, strat),
        "snake" => snake(input, strat),
        other => {
            println!("{}\n\nUnexpected puzzle type: '{}'\n",
                     opts.usage(brief),
                     other)
        }
    };
}
