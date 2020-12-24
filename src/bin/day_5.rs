use std::fs;

#[derive(Debug)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn new(row: usize, col: usize) -> Seat {
        Seat { row, col }
    }

    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn main() {
    let contents = fs::read_to_string("data/day_5.txt").expect("File must be present");
    let seats = contents.split('\n').filter(|s| !s.is_empty()).map(|s| {
        let front_back = &s[..7];
        let left_right = &s[7..];
        let mut row = 0;
        let mut col = 0;

        for (i, chr) in front_back.chars().enumerate() {
            let stride = usize::pow(2, 6 - i as u32);
            if chr == 'B' {
                row += stride;
            }
        }

        for (i, chr) in left_right.chars().enumerate() {
            let stride = usize::pow(2, 2 - i as u32);
            if chr == 'R' {
                col += stride
            }
        }

        Seat::new(row, col)
    });

    let max_id = seats.clone().map(|s| s.id()).max().unwrap();
    println!("{}", max_id);

    let mut previous_seat_id: Option<usize> = None;
    let mut seat_ids: Vec<usize> = seats.map(|s| s.id()).collect();
    seat_ids.sort();
    for seat_id in seat_ids {
        if let Some(previous_seat_id) = previous_seat_id {
            if seat_id - previous_seat_id > 1 {
                println!("{}", seat_id - 1);
            }
        }
        previous_seat_id = Some(seat_id)
    }
}
