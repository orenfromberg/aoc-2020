fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let mut position: (isize, isize) = (0,0);
    let mut orientation: (isize, isize) = (10,1);

    for instruction in instructions.iter() {
        let (i,m) = instruction.split_at(1);
        let mag: usize = m.parse().unwrap();
        match i {
            "N" => {
                for _n in 0..mag {
                    orientation = (orientation.0, orientation.1 + 1isize);
                }
            },
            "S" => {
                for _n in 0..mag {
                    orientation = (orientation.0, orientation.1 - 1isize);
                }
            },
            "E" => {
                for _n in 0..mag {
                    orientation = (orientation.0 + 1isize, orientation.1);
                }
            },
            "W" => {
                for _n in 0..mag {
                    orientation = (orientation.0 - 1isize, orientation.1);
                }
            },
            "L" => {
                for _n in 0..mag/90 {
                    orientation = (-orientation.1, orientation.0);
                }
            },
            "R" => {
                for _n in 0..mag/90 {
                    orientation = (orientation.1, -orientation.0);
                }
            },
            "F" => {
                for _n in 0..mag {
                    position = (position.0 + orientation.0, position.1 + orientation.1);
                }
            },
            _ => {

            }
        }
    }
    println!("manhattan distance is {}", position.0.abs() + position.1.abs());
}
