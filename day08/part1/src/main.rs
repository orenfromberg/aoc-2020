use regex::Regex;
use std::fs;

struct Instruction {
    exec_count: usize,
    operation: String,
    argument: isize,
}

fn main() {
    let re = Regex::new(r"(\S+) (\S+)").unwrap();

    let mut code: Vec<Instruction> = Vec::new();

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

    // instruction pointer
    let mut ip: usize = 0;

    // accumulator
    let mut acc: isize = 0;

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
    }

    println!("acc is {}", acc);
}