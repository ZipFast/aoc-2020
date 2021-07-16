use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{BufReader, BufRead};

pub fn read_lines<P: AsRef<Path>> (pathname: P) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(pathname)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get(vec: &Vec<String>, r: usize, c: usize) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut cnt = 0;
    while x < vec.len() {
        let chars: Vec<char> = vec[x].chars().collect();
        if chars.get(y) == Some(&'#') {
            cnt += 1;
        }
        if y + r >= chars.len() {
            y = (y + r) % chars.len();
        } else {
            y = y + r;
        }
        x = x + c;
    }
    cnt
}

fn main() {
    let filename = "input".to_string();
    let mut vec: Vec<String> = vec![];
    if let Ok(lines) = read_lines(&filename) {
        let mut res = 0;
        for line in lines {
            vec.push(line.unwrap());
        }
        res = get(&vec, 1, 1) * get(&vec, 3, 1) * get(&vec, 5, 1) * get(&vec, 7, 1) * get(&vec, 1, 2);
        println!("{}", res);
    } else {
        panic!("fail to open the file {}", &filename)
    }
}
