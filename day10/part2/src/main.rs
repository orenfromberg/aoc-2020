fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut adapters: Vec<usize> = input.split("\n").map(|l| l.parse().unwrap()).collect();

    adapters.sort_unstable();

    // add wall adapter
    adapters.insert(0, 0);

    // add device adapter
    adapters.push(adapters[adapters.len()-1] + 3);

    // create vec to hold sums of paths
    let mut num_paths = vec![0; adapters[adapters.len()-1]];

    // initialize
    num_paths[0] = 1;

    for adapter in adapters.iter() {
        for n in 1..4 {
            let sum = adapter + n;
            if sum >= num_paths.len() {
                break;
            }
            if adapters.contains(&sum) {
                num_paths[sum] += num_paths[*adapter];
            }
        }
    }

    // get max value in num_paths
    let max: &usize = num_paths.iter().fold(&0, |acc, x| { if x > acc { x } else { acc }});

    println!("{:?}",max);
}