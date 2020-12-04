fn main() {
    let rows: Vec<String> = slurp::read_all_lines("input.txt").unwrap();

    let mut done: bool = false;
    let mut x: usize = 3;
    let mut y: usize = 1;
    let num_rows: usize = rows.len();
    let num_cols: usize = rows[0].len();
    let mut num_trees: u32 = 0;
    while !done {
        if rows[y].chars().nth(x).unwrap() == '#' {
            num_trees += 1;
        }
        y = y + 1;
        x = (x + 3) % num_cols;

        if y >= num_rows {
            done = true;
        }
    }

    println!("num trees = {}", num_trees);
}
