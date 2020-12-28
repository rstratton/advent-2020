use std::ops::{Add, AddAssign, Mul};

enum Command {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl Command {
    fn from_str(s: &str) -> Command {
        let (chr, val) = (s.chars().next().unwrap(), s[1..].parse::<i64>().unwrap());
        match (chr, val) {
            ('N', val) => Command::North(val),
            ('S', val) => Command::South(val),
            ('E', val) => Command::East(val),
            ('W', val) => Command::West(val),
            ('L', val) => Command::Left(val),
            ('R', val) => Command::Right(val),
            ('F', val) => Command::Forward(val),
            _ => unreachable!(),
        }
    }
}

enum Heading {
    North,
    South,
    East,
    West,
}

impl Heading {
    fn turn_left(&self, degrees: i64) -> Self {
        self.rotate(degrees)
    }

    fn turn_right(&self, degrees: i64) -> Self {
        self.rotate(-degrees)
    }

    fn rotate(&self, degrees: i64) -> Self {
        Self::from_degrees(self.to_degrees() + degrees)
    }

    fn from_degrees(degrees: i64) -> Self {
        match degrees.rem_euclid(360) {
            0 => Heading::East,
            90 => Heading::North,
            180 => Heading::West,
            270 => Heading::South,
            _ => panic!("Don't know how to handle {} degrees", degrees),
        }
    }

    fn to_degrees(&self) -> i64 {
        match self {
            Heading::East => 0,
            Heading::North => 90,
            Heading::West => 180,
            Heading::South => 270,
        }
    }

    fn to_vec(&self) -> Vec2 {
        match self {
            Heading::East => Vec2::new(1, 0),
            Heading::North => Vec2::new(0, 1),
            Heading::West => Vec2::new(-1, 0),
            Heading::South => Vec2::new(0, -1),
        }
    }
}

impl Default for Heading {
    fn default() -> Self {
        Heading::East
    }
}

#[derive(Default, Copy, Clone)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<Vec2> for i64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(rhs.x * self, rhs.y * self)
    }
}

struct Ship {
    position: Vec2,
    waypoint: Vec2,
}

impl Ship {
    fn execute(&mut self, command: &Command) {
        match command {
            Command::North(val) => self.waypoint += *val * Heading::North.to_vec(),
            Command::South(val) => self.waypoint += *val * Heading::South.to_vec(),
            Command::East(val) => self.waypoint += *val * Heading::East.to_vec(),
            Command::West(val) => self.waypoint += *val * Heading::West.to_vec(),
            Command::Left(val) => {
                let rotated_x_component = self.waypoint.x * Heading::East.turn_left(*val).to_vec();
                let rotated_y_component = self.waypoint.y * Heading::North.turn_left(*val).to_vec();
                self.waypoint = rotated_x_component + rotated_y_component;
            }
            Command::Right(val) => {
                let rotated_x_component = self.waypoint.x * Heading::East.turn_right(*val).to_vec();
                let rotated_y_component =
                    self.waypoint.y * Heading::North.turn_right(*val).to_vec();
                self.waypoint = rotated_x_component + rotated_y_component;
            }
            Command::Forward(val) => self.position += *val * self.waypoint,
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Vec2::new(0, 0),
            waypoint: Vec2::new(10, 1),
        }
    }
}

fn main() {
    let input = include_str!("../../data/day_12.txt");
    let commands = input
        .lines()
        .map(Command::from_str)
        .collect::<Vec<Command>>();

    let mut ship: Ship = Default::default();
    for command in commands {
        ship.execute(&command);
    }

    println!("{}", ship.position.x.abs() + ship.position.y.abs());
}
