use std::io::BufRead;

pub trait Stored {
    fn load<R: BufRead>(reader: &mut R) -> Self;
}
