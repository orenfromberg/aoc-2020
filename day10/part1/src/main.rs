fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut adapters: Vec<usize> = input.split("\n").map(|l| l.parse().unwrap()).collect();

    adapters.push(0);

    adapters.sort();

    adapters.push(adapters[adapters.len()-1] + 3);

    println!("{:?}", &adapters[..]);

    let mut histogram: [usize;3] = [0;3];

    for i in 1..adapters.len() {
        histogram[adapters[i]-adapters[i-1]-1] += 1;
    }

    println!("{}", histogram[0]*histogram[2]);
}
