use std::collections::HashSet;

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let groups: Vec<String> = input.split("\n\n").map(|l| l.parse().unwrap()).collect();

    let mut count: usize = 0;

    for i in 0..groups.len() {
        let mut questions: HashSet<String> = HashSet::new();
        let mut s = groups[i].clone();
        remove_whitespace(&mut s);

        for j in s.chars() {
            questions.insert(j.to_string());
        }

        count += questions.len();
    }

    println!("{}", count)
}
