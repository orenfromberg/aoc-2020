fn main() {
    let input: Vec<usize> = vec![0,1,4,13,15,12,16];
    let mut memory: Vec<usize> = vec![0;2020];

    let mut last: usize = input[0];
    for n in 0..input.len() {
        memory[last] = n;
        last = input[n];
    }

    for n in input.len()..2020 {
        let prev = memory[last];
        memory[last] = n;
        last = match prev {
            0 => 0,
            _ => n - prev
        }
    }

    println!("{}", last);
}