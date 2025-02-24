use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines, Write};


pub fn save_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn read_to_file(file_path: &str) ->std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())

}