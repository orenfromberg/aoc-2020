use regex::Regex;
use std::fs;

#[derive(Clone)]
struct Instruction {
    exec_count: usize,
    operation: String,
    argument: isize,
}

fn execute(code: &mut Vec<Instruction>) -> bool {
    // instruction pointer
    let mut ip: usize = 0;

    // accumulator
    let mut acc: isize = 0;

    let mut result: bool = false;

    loop {
        if code[ip].exec_count > 0 {
            break;
        }

        match code[ip].operation.as_str() {
            "nop" => {
                code[ip].exec_count += 1;
                ip += 1;
            }
            "acc" => {
                code[ip].exec_count += 1;
                acc += code[ip].argument;
                ip += 1;
            }
            "jmp" => {
                code[ip].exec_count += 1;
                ip = (code[ip].argument + ip as isize) as usize;
            }
            _ => {}
        }

        if ip == code.len() {
            result = true;
            println!("accumulator is {}", acc);
            break;
        }
    }
    result
}

fn main() {
    let re = Regex::new(r"(\S+) (\S+)").unwrap();

    let mut code: Vec<Instruction> = Vec::new();

    // create the object that represents the code
    let input = fs::read_to_string("input.txt").unwrap();
    for cap in re.captures_iter(&input.as_str()) {
        let operation = String::from(&cap[1]);
        let argument = cap[2].parse::<isize>().unwrap();
        code.push(Instruction {
            exec_count: 0,
            operation: operation,
            argument: argument,
        });
    }

    // iterate over the instructions and swap nop for jmp and execute
    for i in 0..code.len() {
        match code[i].operation.as_str() {
            "nop" => {
                let mut new_code = code.to_vec();
                new_code[i].operation = String::from("jmp");
                if execute(&mut new_code) {
                    break;
                }
            },
            "jmp" => {
                let mut new_code = code.to_vec();
                new_code[i].operation = String::from("nop");
                if execute(&mut new_code) {
                    break;
                }
            },
            _ => {}
        }
    }
}