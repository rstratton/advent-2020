use std::{
    fmt,
    ops::{Mul, Range},
};

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn translate_by(&self, offset: &Offset) -> Self {
        Self::new(self.x + offset.dx, self.y + offset.dy)
    }
}

#[derive(Clone, Copy)]
struct Offset {
    dx: isize,
    dy: isize,
}

impl Offset {
    fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }
}

impl Mul<Offset> for isize {
    type Output = Offset;

    fn mul(self, rhs: Offset) -> Self::Output {
        Offset::new(self * rhs.dx, self * rhs.dy)
    }
}

#[derive(PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match self {
            Seat::Floor => ".",
            Seat::Empty => "L",
            Seat::Occupied => "#",
        };
        f.write_str(token)
    }
}

struct Seating {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
}

struct BoundedGrid {
    x_bounds: Range<isize>,
    y_bounds: Range<isize>,
}

impl BoundedGrid {
    fn new(x_bounds: Range<isize>, y_bounds: Range<isize>) -> Self {
        Self { x_bounds, y_bounds }
    }

    fn contains(&self, pos: &Position) -> bool {
        self.x_bounds.contains(&pos.x) && self.y_bounds.contains(&pos.y)
    }
}

impl Seating {
    fn from_str(s: &str) -> Self {
        let lines = s.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().chars().count();
        let seats = lines
            .flat_map(|line| {
                line.chars().map(|chr| match chr {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    _ => unreachable!(),
                })
            })
            .collect();

        Self {
            width,
            height,
            seats,
        }
    }

    fn neighbors(&self, pos: &Position) -> Vec<&Seat> {
        let neighbor_offsets: Vec<Offset> = vec![
            Offset { dx: -1, dy: -1 },
            Offset { dx: 0, dy: -1 },
            Offset { dx: 1, dy: -1 },
            Offset { dx: -1, dy: 0 },
            Offset { dx: 1, dy: 0 },
            Offset { dx: -1, dy: 1 },
            Offset { dx: 0, dy: 1 },
            Offset { dx: 1, dy: 1 },
        ];

        neighbor_offsets
            .iter()
            .flat_map(|offset| self.seat_at(&pos.translate_by(offset)))
            .collect::<Vec<&Seat>>()
    }

    fn neighbors_part_2(&self, pos: &Position) -> Vec<&Seat> {
        let neighbor_directions: Vec<Offset> = vec![
            Offset { dx: -1, dy: -1 },
            Offset { dx: 0, dy: -1 },
            Offset { dx: 1, dy: -1 },
            Offset { dx: -1, dy: 0 },
            Offset { dx: 1, dy: 0 },
            Offset { dx: -1, dy: 1 },
            Offset { dx: 0, dy: 1 },
            Offset { dx: 1, dy: 1 },
        ];

        neighbor_directions
            .iter()
            .flat_map(|direction| {
                ((1 as isize)..)
                    .map(|multiplier| {
                        let neighbor_position = pos.translate_by(&(multiplier * (*direction)));
                        self.seat_at(&neighbor_position)
                    })
                    .take_while(|maybe_seat| maybe_seat.is_some())
                    .flatten()
                    .find(|&seat| seat == &Seat::Empty || seat == &Seat::Occupied)
            })
            .collect::<Vec<&Seat>>()
    }

    fn seat_at(&self, pos: &Position) -> Option<&Seat> {
        let bounded_grid = self.bounded_grid();
        if bounded_grid.contains(pos) {
            Some(&self.seats[self.position_to_index(pos)])
        } else {
            None
        }
    }

    fn bounded_grid(&self) -> BoundedGrid {
        BoundedGrid::new(0..(self.width as isize), 0..(self.height as isize))
    }

    fn position_to_index(&self, pos: &Position) -> usize {
        (pos.y * self.width as isize + pos.x) as usize
    }

    fn index_to_position(&self, idx: usize) -> Position {
        let x = (idx % self.width) as isize;
        let y = (idx / self.width) as isize;
        Position::new(x, y)
    }
}

impl std::cmp::PartialEq for Seating {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        if self.seats.len() != other.seats.len() {
            return false;
        }

        for (s1, s2) in self.seats.iter().zip(other.seats.iter()) {
            if *s1 != *s2 {
                return false;
            }
        }

        true
    }
}

impl fmt::Debug for Seating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars: Vec<char> = Default::default();

        for y in 0..self.height {
            for x in 0..self.width {
                let seat = self
                    .seat_at(&Position::new(x as isize, y as isize))
                    .unwrap();
                let chr = match seat {
                    Seat::Floor => '.',
                    Seat::Empty => 'L',
                    Seat::Occupied => '#',
                };
                chars.push(chr);
            }
            chars.push('\n');
        }

        f.write_str(&chars.into_iter().collect::<String>())
    }
}

impl IntoIterator for Seating {
    type Item = Self;
    type IntoIter = SeatingIter;

    fn into_iter(self) -> Self::IntoIter {
        SeatingIter {
            current_seating: self,
        }
    }
}

struct SeatingIter {
    current_seating: Seating,
}

impl Iterator for SeatingIter {
    type Item = Seating;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_seats: Vec<Seat> = Vec::with_capacity(self.current_seating.seats.len());
        for (idx, seat) in self.current_seating.seats.iter().enumerate() {
            let pos = self.current_seating.index_to_position(idx);
            let neighbors = self.current_seating.neighbors_part_2(&pos);

            let new_seat_state = match &seat {
                Seat::Floor => Seat::Floor,
                Seat::Empty => {
                    let occupied_neighbors_count = neighbors
                        .iter()
                        .filter(|seat| matches!(seat, Seat::Occupied))
                        .count();
                    if occupied_neighbors_count > 0 {
                        Seat::Empty
                    } else {
                        Seat::Occupied
                    }
                }
                Seat::Occupied => {
                    let occupied_neighbors_count = neighbors
                        .iter()
                        .filter(|seat| matches!(seat, Seat::Occupied))
                        .count();
                    if occupied_neighbors_count >= 5 {
                        Seat::Empty
                    } else {
                        Seat::Occupied
                    }
                }
            };

            next_seats.push(new_seat_state);
        }

        let next_seating = Seating {
            seats: next_seats,
            ..self.current_seating
        };

        Some(std::mem::replace(&mut self.current_seating, next_seating))
    }
}

fn main() {
    let input = include_str!("../../data/day_11.txt");
    let initial_seating = Seating::from_str(&input);
    let mut previous_seating = None;
    for seating in initial_seating {
        if let Some(previous_seating) = &previous_seating {
            if seating == *previous_seating {
                break;
            }
        }
        previous_seating = Some(seating)
    }

    let occupied_seats = previous_seating
        .unwrap()
        .seats
        .iter()
        .filter(|s| s == &&Seat::Occupied)
        .count();

    println!("{}", occupied_seats);
}
