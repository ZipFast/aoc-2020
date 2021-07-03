use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn solve() -> i32 {
    let mut vs = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                let num = num.parse::<i32>().unwrap();
                vs.push(num);
            }
        }
        vs.sort();
        for i in 0..vs.len() {
            if i > 0 && vs[i-1] == vs[i] {
                continue
            }
            for j in i+1..vs.len()-1 {
                if j > i+1 && vs[j-1] == vs[j] {
                    continue
                }
                let target = 2020 - vs[i] - vs[j];
                if target <= 0 {
                    continue
                }
                let mut a = j+1;
                let mut b = vs.len()-1;
                while a <= b {
                    let m = (a + b) / 2;
                    if vs[m] == target {
                        return vs[i] * vs[j] * vs[m]; 
                    } else if vs[m] < target {
                        a = m+1;
                    } else {
                        b = m-1;
                    }
                }
            }
        }
        0
    } else {
        0
    }
}
fn main() {
    println!("{}", solve());
}
