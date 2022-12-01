use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input_1.txt") {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
