use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

pub fn read_file_to_vec<T>(path: &str) -> Result<Vec<Vec<T>>, io::Error>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<T> = line
            .split_whitespace()
            .map(|s| s.parse::<T>().expect("Failed to parse"))
            .collect();
        result.push(row)
    }

    Ok(result)
}
