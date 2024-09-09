use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file_line_by_line(filename: &str) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok))
}