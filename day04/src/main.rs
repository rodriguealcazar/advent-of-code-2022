use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum SectionMatch {
    Full,
    Overlap
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].parse() {
        Ok(1) => matching_pairs(&args[2], SectionMatch::Full),
        Ok(2) => matching_pairs(&args[2], SectionMatch::Overlap),
        _ => println!("Chose between part 1 or 2"),
    }
}

fn matching_pairs(input: &str, match_type: SectionMatch) {
    if let Ok(lines) = read_lines(format!("{input}")) {
        let mut full_overlaps = 0;
        for line in lines {
            if let Ok(pairs) = line {
                if let [f_start, f_end, s_start, s_end] =
                    pairs.split(|c| c == ',' || c == '-').collect::<Vec<&str>>()[..] {
                        let f_start: u8 = f_start.parse().unwrap();
                        let f_end: u8 = f_end.parse().unwrap();
                        let s_start: u8 = s_start.parse().unwrap();
                        let s_end: u8 = s_end.parse().unwrap();
                        match match_type {
                            SectionMatch::Full => {
                                if (s_start >= f_start && s_start <= f_end && s_end <= f_end)
                                    || (f_start >= s_start && f_start <= s_end && f_end <= s_end) {
                                        full_overlaps += 1;
                                }
                            },
                            SectionMatch::Overlap => {
                                if (s_start >= f_start && s_start <= f_end)
                                    || (f_start >= s_start && f_start <= s_end) {
                                        full_overlaps += 1;
                                }
                            }
                        }
                    }
            }
        }
        println!("{full_overlaps}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
