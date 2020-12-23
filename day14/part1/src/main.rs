use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split('\n').collect();
    let mask_re = Regex::new(r"mask = ([X10]{36})").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut and_mask: u64 = 0u64;
    let mut or_mask: u64 = 0u64;

    for line in lines.iter() {
        if mask_re.is_match(&line) {
            let caps = mask_re.captures(&line).unwrap();
            let mask = caps.get(1).unwrap().as_str();
            or_mask = mask
                .chars()
                .enumerate()
                .map(|(i, c)| (35 - i, c))
                .fold(0u64, |acc, (n, c)| match c {
                    '1' => 2u64.pow(n as u32) + acc,
                    _ => acc,
                });
            and_mask = !mask
                .chars()
                .enumerate()
                .map(|(i, c)| (35 - i, c))
                .fold(0u64, |acc, (n, c)| match c {
                    '0' => 2u64.pow(n as u32) + acc,
                    _ => acc,
                });
        } else if mem_re.is_match(&line) {
            let caps = mem_re.captures(&line).unwrap();
            let index: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            memory.insert(index as u64, (val & and_mask) | or_mask);
        }
    }

    println!("{}", memory.iter().fold(0u64, |acc, (_, j)| acc + j));
}