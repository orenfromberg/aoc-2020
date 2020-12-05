fn main() {
    let mut ids: Vec<isize> = std::fs::read_to_string("input.txt")
    .unwrap()
    .split("\n")
    .map(|l| get_seat_id(l))
    .collect();

    ids.sort();

    let min: &isize = (&ids[..]).into_iter().min().unwrap();

    for (i, x) in ids.iter().enumerate() {
        let val = *min + i as isize;
        if *x == val {
            continue;
        } else {
            println!("{}", val);
            break;
        }
    }
}

fn get_seat_id(s: &str) -> isize {
    let mut s: String = String::from(s);
    s = s.replace("F","0").replace("B", "1").replace("L", "0").replace("R", "1");
    isize::from_str_radix(s.as_str(), 2).unwrap()
}