use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    if let Ok(lines) = read_lines(format!("{input}")) {
        let mut total_priorities: u64 = 0;
        for line in lines {
            if let Ok(items) = line {
                if items.len() > 0 {
                    total_priorities += value_of_repeated_item(items) as u64;
                }
            }
        }
        println!("{total_priorities}");
    }
}

fn value_of_repeated_item(content: String) -> u8 {
    let middle = content.len() / 2;

    let mut items;

    items = &content[..middle];
    let first_compartment: HashSet<char>;
    first_compartment = HashSet::from_iter(items.chars());

    items = &content[middle..];
    for i in items.chars() {
        if first_compartment.contains(&i) {
            let value = i as u8;
            if value > 64 && value < 91 {
                return value - 64 + 26;
            } else {
                return value - 96;
            }
        }
    }
    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
