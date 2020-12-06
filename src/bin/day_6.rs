use std::collections::HashSet;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("data/day_6.txt").expect("File must be present");
    let group_strings = contents.split("\n\n").filter(|s| s.len() > 0);
    let groups = group_strings.map(|s| s.split("\n"));
    let answers_sets = groups.map(|group| {
        let mut answers_set = HashSet::new();
        for answers in group {
            for answer in answers.chars() {
                answers_set.insert(answer);
            }
        }
        answers_set
    });
    let total: usize = answers_sets.map(|set| set.len()).sum();
    println!("{}", total);
}

fn part2() {
    let contents = fs::read_to_string("data/day_6.txt").expect("File must be present");
    let group_strings = contents.split("\n\n").filter(|s| s.len() > 0);
    let groups = group_strings.map(|s| s.split("\n").filter(|s| s.len() > 0));
    let answers_sets = groups.map(|group| {
        let mut all_answers_set: Option<HashSet<char>> = None;
        for answers in group {
            let mut answers_set = HashSet::new();
            for answer in answers.chars() {
                answers_set.insert(answer);
            }
            match all_answers_set {
                Some(set) => {
                    all_answers_set = Some(set.intersection(&answers_set).cloned().collect());
                }
                None => {
                    all_answers_set = Some(answers_set);
                }
            }
        }
        all_answers_set.unwrap()
    });
    let total: usize = answers_sets.map(|set| set.len()).sum();
    println!("{}", total);
}
