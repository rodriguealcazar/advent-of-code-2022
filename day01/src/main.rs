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
        let mut max = 0;
        let mut current_total = 0;
        for line in lines {
            if let Ok(calories) = line {
                if calories.len() > 0 {
                    current_total += calories.trim().parse::<isize>().unwrap();
                } else {
                    if current_total > max {
                        max = current_total;
                    }
                    current_total = 0;
                }
            }
        }
        println!("Most calories carried: {max}");
    }
}

fn part_two(input: &str) {
    if let Ok(lines) = read_lines(format!("{input}")) {
        let mut top_one = 0;
        let mut top_two = 0;
        let mut top_three = 0;
        let mut current_total = 0;
        for line in lines {
            if let Ok(calories) = line {
                if calories.len() > 0 {
                    current_total += calories.trim().parse::<isize>().unwrap();
                } else {
                    if current_total > top_one {
                        top_three = top_two;
                        top_two = top_one;
                        top_one = current_total;
                    } else if current_total > top_two {
                        top_three = top_two;
                        top_two = current_total;
                    } else if current_total > top_three {
                        top_three = current_total;
                    }
                    current_total = 0;
                }
            }
        }
        let grand_total = top_one + top_two + top_three;
        println!("Most calories carried: {grand_total}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
