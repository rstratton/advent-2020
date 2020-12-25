fn main() {
    let numbers: Vec<usize> = include_str!("../../data/day_9.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let num = part1(&numbers);
    println!("{}", num);
    println!("{}", part2(&numbers, num));
}

fn is_sum_of_any_pair(target: usize, numbers: &[usize]) -> bool {
    for (i, n1) in numbers.iter().enumerate() {
        for n2 in &numbers[(i + 1)..] {
            if n1 + n2 == target {
                return true;
            }
        }
    }

    false
}

fn part1(numbers: &[usize]) -> usize {
    for i in 25..numbers.len() {
        let target = numbers[i];
        let last_25_numbers = &numbers[(i - 25)..i];
        if !is_sum_of_any_pair(target, last_25_numbers) {
            return target;
        }
    }

    panic!("No solution found");
}

fn part2(numbers: &[usize], target: usize) -> usize {
    let slice = find_slice_which_sums_to_target(numbers, target);
    let min = slice.iter().min().unwrap();
    let max = slice.iter().max().unwrap();
    min + max
}

fn find_slice_which_sums_to_target(numbers: &[usize], target: usize) -> &[usize] {
    for i in 0..numbers.len() {
        for slice_size in 2.. {
            let slice = &numbers[i..(i + slice_size)];
            let sum: usize = slice.iter().sum();
            if sum == target {
                return slice;
            }
            if sum > target {
                break;
            }
        }
    }

    panic!("No solution found");
}
