use std::fs;

fn main() {
    let max = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<i32>).sum::<i32>())
        .max()
        .unwrap();

    println!("Part 1: {}", max);

    let mut calories = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<i32>).sum::<i32>())
        .collect::<Vec<_>>();

    calories.sort_by(|a, b| b.cmp(a));

    let max = calories.into_iter().take(3).sum::<i32>();

    println!("Part 2: {}", max);
}
