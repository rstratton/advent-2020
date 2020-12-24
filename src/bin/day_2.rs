use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct PasswordEntry {
    a: usize,
    b: usize,
    character: char,
    password: String,
}

impl PasswordEntry {
    fn is_valid_1(&self) -> bool {
        let char_count = &self
            .password
            .chars()
            .filter(|c| c == &self.character)
            .count();
        char_count >= &self.a && char_count <= &self.b
    }

    fn is_valid_2(&self) -> bool {
        let char_a = &self.password.chars().nth(&self.a - 1).unwrap();
        let char_b = &self.password.chars().nth(&self.b - 1).unwrap();
        (char_a == &self.character || char_b == &self.character) && char_a != char_b
    }
}

impl FromStr for PasswordEntry {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split(' ').collect();
        let a_b: Vec<&str> = components[0].split('-').collect();
        let a = a_b[0].parse().unwrap();
        let b = a_b[1].parse().unwrap();
        let character = components[1].trim_matches(':').chars().next().unwrap();
        let password = components[2].to_string();
        Ok(PasswordEntry {
            a,
            b,
            character,
            password,
        })
    }
}

fn main() {
    let contents = fs::read_to_string("data/day_2.txt").expect("File must be present");

    let passwords: Vec<PasswordEntry> = contents
        .split('\n')
        .filter_map(|s| if !s.is_empty() { s.parse().ok() } else { None })
        .collect();

    let valid_passwords_count_1 = passwords.iter().filter(|p| p.is_valid_1()).count();
    let valid_passwords_count_2 = passwords.iter().filter(|p| p.is_valid_2()).count();

    println!("{}", valid_passwords_count_1);
    println!("{}", valid_passwords_count_2);
}
