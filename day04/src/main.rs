use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = "input_1.txt";
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
                    if (s_start >= f_start && s_start <= f_end && s_end <= f_end)
                        || (f_start >= s_start && f_start <= s_end && f_end <= s_end) {
                        println!("{pairs}");
                        full_overlaps += 1
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
