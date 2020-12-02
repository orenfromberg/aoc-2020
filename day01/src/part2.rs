use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut vec = std::vec::Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                // println!("{}", ip);
                vec.push(value.parse::<i32>().unwrap())
            }
        }
    }

    'outer: for i in 0..vec.len() {
        for j in 0..vec.len() {
            if i == j {
                continue;
            }
            for k in 0..vec.len() {
                if k == j || k ==i {
                    continue;
                }
                if (vec[i] + vec[j] + vec[k]) == 2020 {
                    println!("{} x {} x {} = {}",vec[i],vec[j],vec[k],vec[i]*vec[j]*vec[k]);
                    break 'outer;
                }    
            }

        }
    }

    // while let Some(top) = vec.pop() {
    //     // Prints 3, 2, 1
    //     println!("{}", top);
    // }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
