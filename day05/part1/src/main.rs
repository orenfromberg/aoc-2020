fn main() {
    let max_seat_id = std::fs::read_to_string("input.txt")
    .unwrap()
    .split("\n")
    .map(|l| get_seat_id(l))
    .into_iter()
    .max().unwrap();
    
    println!("{}", max_seat_id);
}

fn get_seat_id(s: &str) -> isize {
    let mut s: String = String::from(s);
    s = s.replace("F","0").replace("B", "1").replace("L", "0").replace("R", "1");
    isize::from_str_radix(s.as_str(), 2).unwrap()
}