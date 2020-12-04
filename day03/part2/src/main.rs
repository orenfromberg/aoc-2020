fn main() {
    let rows: Vec<String> = slurp::read_all_lines("input.txt").unwrap();

    println!("{}", 
    get_num_trees(&rows, 1, 1) *
    get_num_trees(&rows, 3, 1) *
    get_num_trees(&rows, 5, 1) *
    get_num_trees(&rows, 7, 1) *
    get_num_trees(&rows, 1, 2)
);
}

fn get_num_trees(rows: &Vec<String>, _x: usize, _y:usize) -> u32 {
    let num_rows: usize = rows.len();
    let num_cols: usize = rows[0].len();
    let mut num_trees: u32 = 0;
    let mut done: bool = false;

    let mut x: usize = _x;
    let mut y: usize = _y;

    while !done {
        if rows[y].chars().nth(x).unwrap() == '#' {
            num_trees += 1;
        }
        y = y + _y;
        x = (x + _x) % num_cols;

        if y >= num_rows {
            done = true;
        }
    }
    num_trees
}