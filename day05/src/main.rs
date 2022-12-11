use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;



fn main() {
    let args: Vec<String> = env::args().collect();
    part_one(&args[1]);
}

fn part_one(input: &str) {
    if let Ok(mut lines) = read_lines(format!("{input}")) {
        let mut rows = Vec::new();
        let stack_count: u8;
        loop {
            if let Ok(line) = lines.next().expect("") {
                let l: Vec<char> = line.chars().collect();
                if let Some(d) = l[l.len() - 2].to_digit(10) {
                    stack_count = d as u8;
                    break;
                } else {
                    rows.push(l);
                }
            }
        }

        let mut stacks : Vec<Vec<char>> = Vec::new();
        for i in 0..stack_count {
            stacks.push(Vec::new());
        }
        let mut index;
        let mut c: char;
        for row in rows {
            for i in 0..stack_count {
                index = i as usize * 4 + 1;
                c = row[index];
                if c != ' ' {
                    stacks[i as usize].insert(0, row[index]);
                }
            }
        }

        // Consume the empty line
        lines.next();

        let RE = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let mut how_many: u8;
        let mut from: usize;
        let mut to: usize;
        for line in lines {
            if let Ok(step) = line {
                let captures = RE.captures(&step).unwrap();
                how_many = captures.get(1).unwrap().as_str().parse().unwrap();
                from = captures.get(2).unwrap().as_str().parse().unwrap();
                to = captures.get(3).unwrap().as_str().parse().unwrap();

                for m in 0..how_many {
                    let c = stacks[from - 1].pop().unwrap();
                    stacks[to - 1].push(c);
                }
            }
        }
        for s in stacks {
            print!("{}", s.last().unwrap())
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
