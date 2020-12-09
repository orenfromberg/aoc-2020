fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();
    let data: Vec<usize> = input.split("\n").map(|l| l.parse().unwrap()).collect();

    let preamble_length: usize = 25;

    let mut i: usize = preamble_length;

    let mut invalid_num: usize = 0;

    while i < data.len() {
        if !addends_exist_for_sum(data[i], &data[i-preamble_length..i]) {
            invalid_num = data[i];
            break;
        }
        i += 1;
    }

    let length = data.len();
    for win_length in 2..length{
        let mut j = 0;
        while j + win_length < length {
            let section: &[usize] = &data[j..j+win_length];
            if invalid_num == section.iter().fold(0, |acc, x| acc + x) {
                let max: &usize = section.iter().fold(&0, |acc, x| { if x > acc { x } else { acc }});
                let min: &usize = section.iter().fold(&max, |acc, x| {
 if x < acc { x } else { acc }});
                println!("{:?}", section);
                println!("min = {}", min);
                println!("max = {}", max);
                println!("sum is {}", min+max);
            }
            j += 1;
        }
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