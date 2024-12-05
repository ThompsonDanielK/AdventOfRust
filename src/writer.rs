use std::fs::File;
use std::io::{self, BufWriter, Write};

pub fn text_writer(file_path: &str, lines: Vec<String>) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}
