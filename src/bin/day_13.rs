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


    // First find t such that t % 863 == (863 - 72) % 863 counting by 1
    // Next find t such that t % 431 == (431 - 41) % 431 counting by 863 * 1
    // Next find t such that t % 41 == (41 - 0) % 41 counting by 431 * 863 * 1

    let mut modulus = 1u64;
    let mut current_time = 0u64;
    busses.sort_by_key(|(_, frequency)| std::cmp::Reverse(*frequency));
    
    for (offset, frequency) in busses {
        let target_value_modulo_frequency = (frequency - (offset  % frequency)) % frequency;
        for candidate_time in (current_time..).step_by(modulus as usize) {
            if candidate_time % frequency == target_value_modulo_frequency {
                current_time = candidate_time;
                modulus *= frequency;
                break;
            }
        }
    }
    
    current_time
    // for time in times_to_try {
    //     let all_departures_align = busses
    //         .iter()
    //         .all(|(offset, departure_cadence)| (time + *offset) % departure_cadence == 0);

    //     if all_departures_align {
    //         return time;
    //     }
    // }
    // unreachable!();
}
