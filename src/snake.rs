use search::Graph;
use std::fmt;
use std::io::BufRead;
use std::str::FromStr;
use stored::Stored;

pub struct Level {
    width: usize,
    height: usize,
    open: Vec<bool>,
    marked: Vec<bool>,
}
