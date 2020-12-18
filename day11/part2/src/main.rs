fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut rows: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    for row in rows.iter() {
        println!("{}", row);
    }
    println!("\n");

    let mut count = 1;
    loop {
        let prev_rows = rows.clone();

        rows = process(rows);
        if !are_diff(&rows, &prev_rows) {
            break;
        }

        println!("round {}", count);
        for row in rows.iter() {
            println!("{}", row);
        }
        println!("\n");
        count += 1;
    }

    let mut num_occupied = 0;
    for row in rows {
        num_occupied += row.matches("#").count();
    }
    println!("{}", num_occupied);
}

fn are_diff(a: &Vec<String>, b: &Vec<String>) -> bool {
    // do they have the same dimensions
    if (a.len() != b.len()) || (a[0].len() != b[0].len()) {
        return true;
    }
    let mut have_diff: bool = false;
    for i in 0..a.len() {
        have_diff = have_diff || a[i] != b[i];
    }
    have_diff
}

fn process(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();
    for line in lines.iter() {
        new_lines.push(line.clone());
    }

    let num_rows = lines.len();
    let num_cols = lines[0].len();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if let Some(b'L') = lines[row].as_bytes().get(col) {
                // if no adjacent occupied seats
                let y = row as isize;
                let x = col as isize;
                let mut sees_occupied_seats = false;

                'outer: for (j, i) in vec![
                    (-1isize, -1isize),
                    (-1isize, 0isize),
                    (-1isize, 1isize),
                    (0isize, -1isize),
                    (0isize, 1isize),
                    (1isize, -1isize),
                    (1isize, 0isize),
                    (1isize, 1isize),
                ] {
                    for k in 1..num_rows / 2 {
                        let _x = x + (k as isize * i);
                        let _y = y + (k as isize * j);
                        if _x < 0
                            || _x >= (num_cols as isize)
                            || _y < 0
                            || _y >= (num_rows as isize)
                        {
                            continue;
                        }
                        match lines[_y as usize].as_bytes().get((_x) as usize) {
                            Some(b'L') => {
                                continue 'outer;
                            },
                            Some(b'#') => {
                                sees_occupied_seats = true;
                                break 'outer;
                            },
                            _ => {
                                continue;
                            }
                        }
                    }
                }
                
                if !sees_occupied_seats {
                    new_lines[row].replace_range(col..col + 1, "#");
                }
            } else if let Some(b'#') = lines[row].as_bytes().get(col) {
                // if 4 or more adjacent seats are occupied
                let y = row as isize;
                let x = col as isize;
                let mut num_occupied = 0;
                'outer: for (j, i) in vec![
                    (-1isize, -1isize),
                    (-1isize, 0isize),
                    (-1isize, 1isize),
                    (0isize, -1isize),
                    (0isize, 1isize),
                    (1isize, -1isize),
                    (1isize, 0isize),
                    (1isize, 1isize),
                ] {
                    for k in 1..num_rows / 2 {
                        let _x = x + (k as isize * i);
                        let _y = y + (k as isize * j);
                        if _x < 0
                            || _x >= (num_cols as isize)
                            || _y < 0
                            || _y >= (num_rows as isize)
                        {
                            continue;
                        }
                        if let Some(b'#') = lines[(_y) as usize].as_bytes().get((_x) as usize) {
                            num_occupied += 1;
                            continue 'outer;
                        }
                        if let Some(b'L') = lines[(_y) as usize].as_bytes().get((_x) as usize) {
                            continue 'outer;
                        }
                    }
                }
                if num_occupied >= 5 {
                    new_lines[row].replace_range(col..col + 1, "L");
                }
            }
        }
    }
    new_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_not_diff() -> Result<(), String> {
        let a = vec![String::from("L.L"),String::from("LLL"),String::from("L.L")];
        let b = vec![String::from("L.L"),String::from("LLL"),String::from("L.L")];
        assert!(!are_diff(&a, &b));
        Ok(())
    }

    #[test]
    fn test_are_diff() -> Result<(), String> {
        let a = vec![String::from("L.L"),String::from("LLL"),String::from("L.L")];
        let b = vec![String::from("..L"),String::from("LLL"),String::from("L.L")];
        assert!(are_diff(&a, &b));
        Ok(())
    }

    #[test]
    fn test_round1() -> Result<(), String> {
        let mut a = vec![
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL")
        ];
        let b = vec![
            String::from("#.##.##.##"),
            String::from("#######.##"),
            String::from("#.#.#..#.."),
            String::from("####.##.##"),
            String::from("#.##.##.##"),
            String::from("#.#####.##"),
            String::from("..#.#....."),
            String::from("##########"),
            String::from("#.######.#"),
            String::from("#.#####.##"),
        ];
        a = process(a);
        assert!(!are_diff(&a, &b));
        Ok(())
    }

    #[test]
    fn test_round2() -> Result<(), String> {
        let mut a = vec![
            String::from("#.##.##.##"),
            String::from("#######.##"),
            String::from("#.#.#..#.."),
            String::from("####.##.##"),
            String::from("#.##.##.##"),
            String::from("#.#####.##"),
            String::from("..#.#....."),
            String::from("##########"),
            String::from("#.######.#"),
            String::from("#.#####.##"),
        ];
        let b = vec![
            String::from("#.LL.LL.L#"),
            String::from("#LLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLL#"),
            String::from("#.LLLLLL.L"),
            String::from("#.LLLLL.L#"),
        ];
        a = process(a);

        println!("a:\n");
        for row in a.iter() {
            println!("{}", row);
        }
        println!("\n");


        println!("b:\n");
        for row in b.iter() {
            println!("{}", row);
        }
        println!("\n");

        assert!(!are_diff(&a, &b));
        Ok(())
    }

    #[test]
    fn test_round3() -> Result<(), String> {
        let mut a = vec![
            String::from("#.LL.LL.L#"),
            String::from("#LLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLL#"),
            String::from("#.LLLLLL.L"),
            String::from("#.LLLLL.L#"),
        ];
        let b = vec![
            String::from("#.L#.##.L#"),
            String::from("#L#####.LL"),
            String::from("L.#.#..#.."),
            String::from("##L#.##.##"),
            String::from("#.##.#L.##"),
            String::from("#.#####.#L"),
            String::from("..#.#....."),
            String::from("LLL####LL#"),
            String::from("#.L#####.L"),
            String::from("#.L####.L#"),
        ];
        let truth = a.clone();
        a = process(a);

        println!("truth:\n");
        for row in truth.iter() {
            println!("{}", row);
        }
        println!("\n");


        println!("b:\n");
        for row in b.iter() {
            println!("{}", row);
        }
        println!("\n");

        assert!(!are_diff(&a, &b));
        Ok(())
    }
}
