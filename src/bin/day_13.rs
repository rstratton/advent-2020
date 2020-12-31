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
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(idx, s)| (idx as u64, s.parse::<u64>().unwrap()))
        .collect::<Vec<(u64, u64)>>();

    for (offset, frequency) in busses.sort_by_key(|(_, frequency)| std::cmp::Reverse(frequency)) {}
    let (offset, greatest_departure_cadence) = busses
        .iter()
        .max_by_key(|(_, departure_cadence)| departure_cadence)
        .unwrap();

    let times_to_try =
        ((*greatest_departure_cadence - *offset)..).step_by(*greatest_departure_cadence as usize);

    println!("{:?}", busses);
    println!("{:?} -- {:?}", offset, greatest_departure_cadence);
    println!("{:?}", times_to_try.take(10).collect::<Vec<u64>>());
    0
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
