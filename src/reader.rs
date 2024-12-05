use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn text_reader(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
