fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split('\n').collect();
    let pairs: Vec<(usize, usize)> = lines[1]
        .split(',')
        .enumerate()
        .filter(|item| item.1 != "x")
        .map(|item| (item.0, item.1.parse::<usize>().unwrap()))
        .collect();

    let mut timestamp: usize = 1;
    let mut wait_time: usize = 1;
    for (bus_num, bus_minutes) in pairs {
        if bus_minutes == 0 {
            continue;
        }
        loop {
            if (timestamp + bus_num) % bus_minutes == 0 {
                wait_time *= bus_minutes;
                break;
            }
            timestamp += wait_time;
        }
    }
    println!("Part2) {}", timestamp);
}
