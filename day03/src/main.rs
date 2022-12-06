use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].parse() {
        Ok(1) => part_one(&args[2]),
        Ok(2) => part_two(&args[2]),
        _ => println!("Chose between part 1 or 2"),
    }

}

fn part_one(input: &str) {
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

fn part_two(input: &str) {
    let lines = read_lines(format!("{input}"))
        .expect("Failed to read input file.");

    let mut priorities: usize = 0;
    let mut item_counts: HashMap<char, u8> = HashMap::new();
    let mut elves = 1;
    for line in lines {
        if let Ok(items) = line {
            let uniques: HashSet<char> = HashSet::from_iter(items.chars());
            for i in uniques {
                match item_counts.get(&i) {
                    Some(value) => {
                        if *value == 2 {
                            priorities += value_of_item(i) as usize;
                        } else {
                            item_counts.insert(i, 2);
                        }
                    },
                    None => {
                        item_counts.insert(i, 1);
                    }
                };
            }
        }
        if elves % 3 == 0 {
            item_counts.clear();
        }
        elves += 1;
    }
    println!("Sum of priorities: {priorities}");
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
            return value_of_item(i)
        }
    }
    return 0;
}

fn value_of_item(item: char) -> u8 {
    let value = item as u8;
    if value > 64 && value < 91 {
        return value - 64 + 26;
    } else {
        return value - 96;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
