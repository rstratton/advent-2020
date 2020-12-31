fn main() {
    let input = include_str!("../../data/day_13.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let earliest_possible_departure_time = lines.next().unwrap().parse::<u64>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for time in earliest_possible_departure_time.. {
        for bus in &busses {
            if time % bus == 0 {
                return (time - earliest_possible_departure_time) * bus;
            }
        }
    }
    unreachable!();
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let _earliest_possible_departure_time = lines.next().unwrap().parse::<u64>().unwrap();
    let mut busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(idx, s)| (idx as u64, s.parse::<u64>().unwrap()))
        .collect::<Vec<(u64, u64)>>();

    let mut modulus = 1u64;
    let mut current_time = 0u64;
    busses.sort_by_key(|(_, frequency)| std::cmp::Reverse(*frequency));

    for (offset, frequency) in busses {
        let target_value_modulo_frequency = (frequency - (offset % frequency)) % frequency;
        for candidate_time in (current_time..).step_by(modulus as usize) {
            if candidate_time % frequency == target_value_modulo_frequency {
                current_time = candidate_time;
                modulus *= frequency;
                break;
            }
        }
    }

    current_time
}
