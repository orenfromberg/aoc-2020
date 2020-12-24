use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split('\n').collect();
    let mask_re = Regex::new(r"mask = ([X10]{36})").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: &str = "";

    for line in lines.iter() {
        if mask_re.is_match(&line) {
            let caps = mask_re.captures(&line).unwrap();
            mask = caps.get(1).unwrap().as_str();
        } else if mem_re.is_match(&line) {
            let caps = mem_re.captures(&line).unwrap();
            let index: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();

            let address = format!("{:0>36b}", index).chars().rev().collect::<Vec<char>>();
            let mask_slice = &mask[mask.len()-address.len()..mask.len()];
            let _foo = address.iter().zip(mask_slice.to_string().chars().rev()).map(|(a,b)| match b {
                'X' => 'X',
                '1' => '1',
                '0' =>  *a,
                _ => panic!("error")
            }).collect::<Vec<char>>();

            let addresses: Vec<u64> = process_mask(&_foo.iter().collect()).iter().map(|x| get_value(x)).collect();

            for addr in addresses {
                memory.insert(addr, val);
            }
        }
    }

    println!("{}", memory.iter().fold(0u64, |acc, (_, j)| acc + j));
}

fn process_mask(mask: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    match mask.find('X') {
        Some(_) => {
            let mut _a = mask.clone().to_string();
            let mut _b = mask.clone().to_string();
            for j in process_mask(&_a.replacen("X", "0", 1)).iter() {
                result.push(j.clone());
            }
            for k in process_mask(&_b.replacen("X", "1", 1)).iter() {
                result.push(k.clone());
            }
        }
        None => {
            result.push(mask.clone());
        }
    }
    result
}

fn get_value(mask: &String) -> u64 {
    mask.chars()
        .enumerate()
        .fold(0u64, |acc, (n, c)| {
            acc + (c as u64 - '0' as u64) * 2u64.pow(n as u32)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_mask() -> Result<(), String> {
        let a = process_mask(&String::from("000000000000000000000000000000X1001X"));

        assert_eq!(
            a,
            vec![
                String::from("000000000000000000000000000000010010"),
                String::from("000000000000000000000000000000010011"),
                String::from("000000000000000000000000000000110010"),
                String::from("000000000000000000000000000000110011")
            ]
        );
        Ok(())
    }

    #[test]
    fn test_get_value() -> Result<(), String> {
        assert_eq!(
            get_value(&String::from("010101")),
            42u64
        );
        Ok(())
    }
}
