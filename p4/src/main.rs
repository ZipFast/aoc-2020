use std::path::Path;
use std::io::{Lines, BufReader, BufRead, Result};
use std::fs::File;
use std::collections::HashMap;
use std::borrow::Borrow;

fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn check_data(front: &str, back: &str) -> bool {
    match front {
        "byr" => {
            if back.len() != 4 {
                return false;
            } else {
                return back >= "1920" && back <= "2002";
            }
        }
        "iyr" => {
            if back.len() != 4 {
                return false;
            } else {
                return back >= "2010" && back <= "2020";
            }
        }
        "eyr" => {
            if back.len() != 4 {
                return false;
            } else {
                return back >= "2020" && back <= "2030";
            }
        }
        "hgt" => {
            if back.ends_with("cm") {
                let num = &back[..back.find("cm").unwrap()];
                if num.len() == 3 {
                    return num >= "150" && num <= "193";
                }
                return false;
            }
            if back.ends_with("in") {
                let num = &back[..back.find("in").unwrap()];
                if num.len() == 2 {
                    return num >= "59" && num <= "76";
                }
                return false;
            }
            return false;
        }
        "hcl" => {
            if back.starts_with("#") {
                let remain = &back[1..];
                if remain.len() == 6 {
                    for b in remain.as_bytes() {
                        if !((b >= &b'0' && b <= &b'9') || (b >= &b'a' && b <= &b'f')) {
                            return false;
                        }
                    }
                    return true;
                }
                return false;
            }
            return false;
        }
        "ecl" => {
            if back == "amb" || back == "blu" || back =="brn" || back == "gry" || back == "grn" || back == "hzl" || back == "oth" {
                return true;
            }
            return false;
        }
        "pid" => {
            if back.len() == 9 {
                for b in back.as_bytes() {
                    if !(b >= &b'0' && b <= &b'9') {
                        return false;
                    }
                }
                return true;
            }
            return false;
        }
        "cid" => true,
        _ =>
            false,

    }
}
fn main() {
    let mut hashMap = HashMap::new();
    hashMap.insert(String::from("byr"), 0);
    hashMap.insert(String::from("iyr"), 0);
    hashMap.insert(String::from("eyr"), 0);
    hashMap.insert(String::from("hgt"), 0);
    hashMap.insert(String::from("hcl"), 0);
    hashMap.insert(String::from("ecl"), 0);
    hashMap.insert(String::from("pid"), 0);

    let mut count = 0;
    let mut valid_cnt = 0;
    let mut total = 0;
    let mut flag = true;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 {
                    total += 1;
                    println!("{:?}", hashMap);
                    if count == 7 && flag {
                        valid_cnt += 1;
                    }
                    flag = true;
                    count = 0;
                    for (_, value) in &mut hashMap {
                        *value = 0;
                    }
                } else {
                    for field in line.split_whitespace() {
                        let (front,back) = field.split_at(field.find(":").unwrap());
                        let back = &back[1..];
                        println!("{}:{}", front, back);
                        if !check_data(front, back) {
                            flag = false;
                        }
                        if hashMap.contains_key(front) && hashMap.get(front) == Some(&0) {
                            hashMap.insert(String::from(front), hashMap.get(front).unwrap()+1);
                            count += 1;
                        }
                    }
                }
            }
        }
        println!("{}", total);
        println!("{}", valid_cnt);
    }
}
