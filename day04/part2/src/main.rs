fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = input.split("\n\n").map(|l| l.parse().unwrap()).collect();

    let byr_re = regex::Regex::new(r"byr:(19[2-9][0-9]|200[0-2])\b").unwrap();
    let iyr_re = regex::Regex::new(r"iyr:(201[0-9]|2020)\b").unwrap();
    let eyr_re = regex::Regex::new(r"eyr:(202[0-9]|2030)\b").unwrap();
    let hgt_re = regex::Regex::new(r"hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\b").unwrap();
    let hcl_re = regex::Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
    let ecl_re = regex::Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid_re = regex::Regex::new(r"pid:[0-9]{9}\b").unwrap();

    let mut num_valid: usize = 0;
        for i in 0..lines.len() {
            let s = (&lines[i]).to_lowercase();
            if  hgt_re.is_match(s.as_str()) &&
                eyr_re.is_match(s.as_str()) &&
                ecl_re.is_match(s.as_str()) &&
                pid_re.is_match(s.as_str()) &&
                hcl_re.is_match(s.as_str()) &&
                byr_re.is_match(s.as_str()) &&
                iyr_re.is_match(s.as_str())
                {
                    num_valid += 1;
                }
        }

        println!("num_valid = {}", num_valid);
}
