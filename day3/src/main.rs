use std::fs;

fn main() {
    let result1: u32 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(a, b)| a.chars().find(|x| b.contains(*x)).unwrap())
        .map(char_to_value)
        .sum();

    println!("Part 1: {}", result1);

    let result2: u32 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .chunks_mut(3)
        .map(find_intersection_value)
        .map(char_to_value)
        .sum();

    println!("Part 2: {}", result2);
}

fn char_to_value(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 'A' as u32 + 27;
    }
    c as u32 - 'a' as u32 + 1
}

fn find_intersection_value(lists: &mut [Vec<char>]) -> char {
    let (intersection, others) = lists.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|e| other.contains(e));
    }

    intersection[0]
}
