fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let groups: Vec<String> = input.split("\n\n").map(|l| l.parse().unwrap()).collect();

    let mut count: usize = 0;

    for i in 0..groups.len() {
        let responses: Vec<String> = groups[i].clone().split("\n").map(|l| l.parse().unwrap()).collect();

        let mut counts = [0;26];

        for j in 0..responses.len() {
            for k in responses[j].chars() {
                let idx = k as usize - 97;
                counts[idx] += 1;
            }
        }

        println!("{:?}", counts);

        for j in 0..26 {
            if counts[j] == responses.len() {
                count += 1;
            }
        }
    }

    println!("{}", count)
}
