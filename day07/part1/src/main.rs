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
            println!("{}", &jcaps["outerbag"]);

            let mut inner_bags: Vec<String> = Vec::new();

            for kcaps in inner_re.captures_iter(rules[i].as_str()) {
                let count: u32 = (&kcaps["count"]).parse::<u32>().unwrap();
                inner_bags.push(String::from(&kcaps["innerbag"]))
            }
            rules_map.insert(String::from(&jcaps["outerbag"]), inner_bags);
        }
    }

    println!("{:?}", rules_map);

    let mut count = 0;

    for key in rules_map.keys() {
        if has_shiny_gold_bag(&rules_map, key) {
            count += 1;
        }
    }

    println!("{} bags contain shiny gold", count);
}

fn has_shiny_gold_bag(m: &HashMap<String, Vec<String>>, s: &String) -> bool {
    if m.get(s).unwrap().contains(&String::from("shiny gold")) {
        return true;
    }

    for bag in m.get(s).unwrap() {
        if has_shiny_gold_bag(m, bag) {
            return true;
        }
    }
    
    false
}
