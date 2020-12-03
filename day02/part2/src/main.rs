fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let passwords: Vec<String> = input.split("\n").map(|l| l.parse().unwrap()).collect();

    use regex::Regex;
    let re = Regex::new(r"^(\d+)-(\d+) (\D): (\D+)$").unwrap();

    let mut num_valid = 0;

    for i in 0..passwords.len() {
        let caps = re.captures(&passwords[i]).unwrap();

        let pos1: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let pos2: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let letter = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();
        let letter_at_pos1 = password.chars().nth(pos1-1).unwrap().to_string();
        let letter_at_pos2 = password.chars().nth(pos2-1).unwrap().to_string();
        

        if (letter_at_pos1 == letter) ^ (letter_at_pos2 == letter) {
            num_valid += 1;
        }
    }

    println!("{} valid passwords", num_valid)
}
