fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split('\n').collect();
    let departure_time: usize = lines[0].parse().unwrap();
    let answer = lines[1]
    .split(',')
    .filter(|item| item != &"x")
    .map(|item| item.parse::<usize>().unwrap())
    .map(|item| (item, (((departure_time / item) + 1) * item) - departure_time))
    .min_by_key(|pair| pair.1)
    .unwrap();

    println!("{}", answer.0 * answer.1);
}
