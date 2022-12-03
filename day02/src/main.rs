use std::collections::HashMap;
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

fn part_two(input: &str) {
    let mut winners = HashMap::new();
    winners.insert("A", "B");
    winners.insert("B", "C");
    winners.insert("C", "A");

    let mut losers = HashMap::new();
    losers.insert("A", "C");
    losers.insert("B", "A");
    losers.insert("C", "B");

    let mut points = HashMap::new();
    points.insert("A", 1);
    points.insert("B", 2);
    points.insert("C", 3);

    if let Ok(lines) = read_lines(format!("{input}")) {
        let mut total: u32 = 0;
        for parsed_line in lines {
            if let Ok(line) = parsed_line {
                if line.len() == 0 {
                    continue;
                }

                let plays = line.split_whitespace().collect::<Vec<&str>>();
                let elf = *plays.get(0).unwrap();
                let play = *plays.get(1).unwrap();

                let score: u32;
                if play == "X" {
                    let mine = losers.get(elf).unwrap();
                    score = *points.get(mine).unwrap();
                } else if play == "Y" {
                    score = 3 + *points.get(elf).unwrap();
                } else {
                    let mine = winners.get(elf).unwrap();
                    score = 6 + *points.get(mine).unwrap();
                }
                total += score;
            }
        }
        println!("Final score: {total}");
    } else {
        println!("Nothing to read in '{input}'.");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
