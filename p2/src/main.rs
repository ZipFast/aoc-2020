use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut success = 0;
        for line in lines {
            let line = &line.expect("error");
            println!("line is {}", line);
            let ss = line.split(":").collect::<Vec<&str>>();
            assert_eq!(ss.len(), 2);
            let (front, back) = (ss[0].trim(), ss[1].trim());
            let last_char = front.chars().last().unwrap();
            let mut cnt = 0;
            for c in back.chars() {
                if c == last_char {
                    cnt += 1;
                }
            }
            println!("the target char is {}, the count is {}", last_char, cnt);
            let lower = &front[..front.find("-").unwrap()];
            let upper = &front[front.find("-").unwrap()+1..front.len()-2];
            println!("lower value is {}, upper value is {}", lower, upper);
            let (l, u) = (<i32 as FromStr>::from_str(lower).unwrap(), <i32 as FromStr>::from_str(upper).unwrap());
            if cnt >= l && cnt <= u {
                success += 1;
            }
        }
        println!("the total success is {}", success);
    } else {
        panic!("cannot open the file");
    }
}
