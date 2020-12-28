use itertools::Itertools;

fn main() {
    let mut output_joltages = include_str!("../../data/day_10.txt")
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let max_joltage = *output_joltages.iter().max().unwrap();
    output_joltages.push(max_joltage + 3);
    output_joltages.push(0);
    output_joltages.sort_unstable();
    let differences = output_joltages
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<u64>>();

    // Part 1
    let differences_of_1 = differences.iter().filter(|&d| *d == 1).count();
    let differences_of_3 = differences.iter().filter(|&d| *d == 3).count();
    println!("{}", differences_of_1 * differences_of_3);

    // Part 2

    // It turns out that there are never differences of 2 in the puzzle input.
    // This makes the puzzle a lot easier.  We know whenever we see a difference
    // of 3 that the adapter on either end of the difference cannot be omitted because
    // we would then have a joltage gap greather than 3 remaining, which is illegal.
    // Thus the only adapters we can consider omitting are those which have a difference
    // of 1 on each side.
    //
    // For example, imagine we have the following joltage adapters (the top row represents
    // joltage values and the bottom row has a `*` if we have an adapter rated for that
    // joltage):
    //
    // |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 |
    // +----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
    // |  * |  * |  * |    |    |  * |  * |  * |  * |  * |    |    |  * |  * |  * |    |    |  * |
    //
    // In this example, the adapters with joltages 1, 6, 7, 8, and 13 could potentially be
    // omitted, while the adapters with joltages of 0, 2, 5, 9, 12, 14, and 17 cannot be
    // omitted without violating the constraint that our adapter chain never has a difference
    // greater than 3.  If we express the above diagram as a sequence of joltage differences,
    // we get:
    //
    // [1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 3]
    //
    // For each contiguous sequence of differences of 1, we can take the number of 1's in the
    // sequence and subtract 1; this yields the number of joltage adapters which could possibly
    // be omitted.  For each such contiguous sequence, the choice to omit or include an adapter
    // is independent from the choice to omit adapters in other sequences.  Thus if we count the
    // number of legal combinations within each subsequence and multiply them together we'll
    // arrive at the total number of legal combinations for the entire sequence.

    let total_adapter_combinations = contiguous_omissible_adapters(&differences)
        .iter()
        .map(|adapter_count| adapter_combinations(1, *adapter_count))
        .product::<u64>();
    println!("{}", total_adapter_combinations);
    println!("{}", adapter_combinations(1, 1));
    println!("{}", adapter_combinations(1, 2));
    println!("{}", adapter_combinations(1, 3));
    println!("{}", adapter_combinations(1, 4));
    println!("{}", adapter_combinations(1, 5));
    println!("{}", adapter_combinations(1, 6));
    println!("{}", adapter_combinations(1, 7));
    println!("{}", adapter_combinations(1, 8));
}

// Given a slice of adapter rating differences, return a vec where
// each element represents a count of contiguous omissible adapters
// in a subsequence of joltage adapters.  There will be many such
// subsequences, thus we return a vec of values.
//
// For a concrete example, consider the following sequence of joltage
// adapters:
//
// |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 |
// +----+----+----+----+----+----+----+----+----+----+----+----+----+----+
// |  * |  * |  * |    |    |  * |  * |  * |  * |  * |    |    |  * |  * |
//
// This is represented by this sequence of differences: [1, 1, 3, 1, 1, 1, 1].
// The first subsequence of contiguous adapters by joltage rating
// (at joltages [0, 2] inclusive) has one omissible adapter, at
// joltage rating 1.
//
// The second (and final) subsequence of contiguous
// adapters (at joltages [5, 9] inclusive) has 3 omissible adapters,
// with ratings 6, 7, and 8.
//
// The joltage adapters with ratings 12 and 13 are not omissible.
//
// Thus `contiguous_omissible_adapters` would
// return `vec![1, 3]`.
fn contiguous_omissible_adapters(differences: &[u64]) -> Vec<u64> {
    let mut results: Vec<u64> = Default::default();
    let mut contiguous_ones_count = 0;

    for difference in differences {
        if *difference == 1 {
            contiguous_ones_count += 1;
        }

        if *difference == 3 {
            // We only want to consider cases where we have
            // at least two differences of 1.  If we have just
            // one difference of 1, neither adapter on either
            // side of the difference is omissible.
            if contiguous_ones_count > 1 {
                results.push(contiguous_ones_count - 1);
            }
            contiguous_ones_count = 0
        }
    }

    results
}

// Recursively count the number of possible configurations which are legal for
// a contiguous series of adapters.  `lower_joltage_difference` represents the
// joltage gap between the current adapter and the last adapter which we did not
// omit.  `remaining_adapters` counts how many contiguous adapters we have to
// decide whether to omit or not, including the current adapter.
fn adapter_combinations(lower_joltage_difference: u64, remaining_adapters: u64) -> u64 {
    if remaining_adapters == 0 {
        return 1;
    }

    if lower_joltage_difference >= 3 {
        return adapter_combinations(1, remaining_adapters - 1);
    }

    adapter_combinations(1, remaining_adapters - 1)
        + adapter_combinations(lower_joltage_difference + 1, remaining_adapters - 1)
}
