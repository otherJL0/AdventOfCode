use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "data/day_01/input";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut values: Vec<isize> = vec![];

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
