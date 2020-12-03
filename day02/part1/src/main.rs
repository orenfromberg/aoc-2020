fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let passwords: Vec<String> = input.split("\n").map(|l| l.parse().unwrap()).collect();

    use regex::Regex;
    let re = Regex::new(r"^(?P<lower>\d+)-(?P<upper>\d+) (?P<letter>\D): (?P<password>\D+)$").unwrap();

    let mut num_valid = 0;

    for i in 0..passwords.len() {
        let caps = re.captures(&passwords[i]).unwrap();

        let mut count = 0;
        for c in caps["password"].chars() {
            if c.to_string() == caps["letter"] {
                count = count + 1;
            }
        }

        println!("{} has the letter {} {} times", &caps["password"], &caps["letter"], count);

        if count >= caps["lower"].parse().unwrap() && count <= caps["upper"].parse().unwrap() {
            num_valid = num_valid + 1;
        }
    }

    println!("{} valid passwords", num_valid)
}
