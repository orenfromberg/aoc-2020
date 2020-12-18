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
                let mut no_adj_occupants = false;
                for (j, i) in vec![
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    if x + i < 0
                        || x + i >= (num_cols as isize)
                        || y + j < 0
                        || y + j >= (num_rows as isize)
                    {
                        continue;
                    }
                    no_adj_occupants = no_adj_occupants
                        || match lines[(y + j) as usize].as_bytes().get((x + i) as usize) {
                            Some(b'L') => false,
                            Some(b'#') => true,
                            _ => false,
                        }
                }
                if !no_adj_occupants {
                    new_lines[row].replace_range(col..col + 1, "#");
                }
            } else if let Some(b'#') = lines[row].as_bytes().get(col) {
                // if 4 or more adjacent seats are occupied
                let y = row as isize;
                let x = col as isize;
                let mut num_occupied = 0;
                for (j, i) in vec![
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    if x + i < 0
                        || x + i >= (num_cols as isize)
                        || y + j < 0
                        || y + j >= (num_rows as isize)
                    {
                        continue;
                    }
                    num_occupied += match lines[(y + j) as usize].as_bytes().get((x + i) as usize) {
                        Some(b'L') => 0,
                        Some(b'#') => 1,
                        _ => 0,
                    }
                }
                if num_occupied >= 4 {
                    new_lines[row].replace_range(col..col + 1, "L");
                }
            }
        }
    }
    new_lines
}
