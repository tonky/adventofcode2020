use std::fs;

fn main() {
    let lines = fs::read_to_string("input").unwrap();

    let trees = traverse(&lines, 3, 1)
        * traverse(&lines, 1, 1)
        * traverse(&lines, 5, 1)
        * traverse(&lines, 7, 1)
        * traverse(&lines, 1, 2);

    println!("{} trees", trees);
}

fn traverse(input: &String, right: usize, down: usize) -> usize {
    input
        .lines()
        .step_by(down)
        .enumerate()
        .filter(|(no, line)| line.chars().nth((no * right) % line.len()).unwrap() == '#')
        .count()
}
