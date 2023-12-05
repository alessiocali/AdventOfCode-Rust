use std::fs::File;
use std::io::{ BufReader, BufRead };

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IOError: {0}")]
    IOError(String)
}

pub fn clamp<T>(num: T, min: T, max: T) -> T
where T: Ord
{
    std::cmp::max(std::cmp::min(num, max), min)
}

pub fn read_file(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path).map_err(|e| Error::IOError(e.to_string()))?;
    let line_result: Result<Vec<_>, _> = BufReader::new(file).lines().collect();
    Ok(line_result.map_err(|e| Error::IOError(e.to_string()))?)
}