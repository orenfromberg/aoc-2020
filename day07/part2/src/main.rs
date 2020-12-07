use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let rules: Vec<String> = input.split("\n").map(|l| l.parse().unwrap()).collect();

    let mut rules_map: HashMap<String, Vec<String>> = HashMap::new();

    let outer_re = Regex::new(r"(?P<outerbag>.*) bags contain").unwrap();
    let inner_re = Regex::new(r"(?P<count>\d+) (?P<innerbag>[a-z ]*) bag[s]*[,|.]").unwrap();

    for i in 0..rules.len() {
        for jcaps in outer_re.captures_iter(rules[i].as_str()) {
            let mut inner_bags: Vec<String> = Vec::new();
            for kcaps in inner_re.captures_iter(rules[i].as_str()) {
                let count: u32 = (&kcaps["count"]).parse::<u32>().unwrap();
                for _l in 0..count {
                    inner_bags.push(String::from(&kcaps["innerbag"]))
                }
            }
            rules_map.insert(String::from(&jcaps["outerbag"]), inner_bags);
        }
    }

    println!("{}", count_bags(&rules_map, &String::from("shiny gold")));
}

fn count_bags(m: &HashMap<String, Vec<String>>, s: &String) -> u32 {
    let mut count: u32 = 0;
    for bag in m.get(s).unwrap() {
        count += count_bags(m, bag);
    }
    count + m.get(s).unwrap().len() as u32
}