use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Write};
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ddp <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);

    if !path.exists() {
        eprintln!("File not found: {}", filename);
        std::process::exit(1);
    }

    let content = fs::read_to_string(filename)?;
    let mut lines: HashSet<String> = HashSet::new();
    let unique_lines: Vec<String> = content
        .lines()
        .filter(|line| lines.insert(line.to_string()))
        .map(|s| s.to_string())
        .collect();

    let mut file = File::create(filename)?;
    for line in unique_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
