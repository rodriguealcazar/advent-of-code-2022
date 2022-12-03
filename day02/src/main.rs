use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut winners = HashMap::new();
    winners.insert("A", "Y");
    winners.insert("B", "Z");
    winners.insert("C", "X");

    let mut draws = HashMap::new();
    draws.insert("A", "X");
    draws.insert("B", "Y");
    draws.insert("C", "Z");

    let mut points = HashMap::new();
    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);

    let input = input_file();

    if let Ok(lines) = read_lines(format!("{input}")) {
        let mut elf: &str;
        let mut mine: &str;
        let mut total = 0;
        for parsed_line in lines {
            if let Ok(line) = parsed_line {
                if line.len() == 0 {
                    continue;
                }

                let plays = line.split_whitespace().collect::<Vec<&str>>();
                elf = *plays.get(0).unwrap();
                mine = *plays.get(1).unwrap();

                let score = points.get(mine).unwrap();
                total += score;

                if *winners.get(elf).unwrap() == mine {
                    total += 6;
                } else if *draws.get(elf).unwrap() == mine {
                    total += 3;
                }
            }
        }
        println!("Final score: {total}");
    } else {
        println!("Nothing to read in '{input}'.");
    }
}

fn input_file() -> String {
    let args: Vec<String> = env::args().collect();

    return String::from(args.get(1).unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
