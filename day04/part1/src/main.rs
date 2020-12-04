fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = input.split("\n\n").map(|l| l.parse().unwrap()).collect();

    let hgt_re = regex::Regex::new(r"hgt:(\S+)").unwrap();
    let eyr_re = regex::Regex::new(r"eyr:(\S+)").unwrap();
    let ecl_re = regex::Regex::new(r"ecl:(\S+)").unwrap();
    let pid_re = regex::Regex::new(r"pid:(\S+)").unwrap();
    let hcl_re = regex::Regex::new(r"hcl:(\S+)").unwrap();
    let byr_re = regex::Regex::new(r"byr:(\S+)").unwrap();
    let iyr_re = regex::Regex::new(r"iyr:(\S+)").unwrap();

    let mut num_valid: usize = 0;
    for i in 0..lines.len() {
        if hgt_re.is_match(&lines[i])
            && eyr_re.is_match(&lines[i])
            && ecl_re.is_match(&lines[i])
            && pid_re.is_match(&lines[i])
            && hcl_re.is_match(&lines[i])
            && byr_re.is_match(&lines[i])
            && iyr_re.is_match(&lines[i])
        {
            num_valid += 1;
        }
    }

    println!("num_valid = {}", num_valid);
}
