fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();
    let data: Vec<usize> = input.split("\n").map(|l| l.parse().unwrap()).collect();

    let preamble_length: usize = 25;

    let mut i: usize = preamble_length;

    while i < data.len() {
        if !addends_exist_for_sum(data[i], &data[i-preamble_length..i]) {
            println!("{}", data[i]);
            return;
        }
        i += 1;
    }
}

fn addends_exist_for_sum(sum: usize, addends: &[usize]) -> bool {
    for i in addends.iter() {
        if i > &sum {
            continue;
        }
        let n = sum - i;
        if addends.contains(&n) {
            return true
        }
    }
    false
}