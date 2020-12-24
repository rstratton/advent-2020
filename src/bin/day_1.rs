use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day_1.txt").expect("File must be present");

    let expenses: Vec<i32> = contents
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("{}", part1(&expenses).unwrap());
    println!("{}", part2(&expenses).unwrap());
}

fn part1(expenses: &[i32]) -> Option<i32> {
    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return Some(expenses[i] * expenses[j]);
            }
        }
    }
    None
}

fn part2(expenses: &[i32]) -> Option<i32> {
    for i in 0..expenses.len() {
        for j in (i + 2)..expenses.len() {
            for k in (i + 1)..j {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    return Some(expenses[i] * expenses[j] * expenses[k]);
                }
            }
        }
    }
    None
}
