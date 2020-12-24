use std::collections::HashSet;
use std::fs;

enum ParserState {
    Key,
    Val,
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn with_key_val(mut self, key: &str, val: &str) -> Passport {
        let val = Some(val.to_string());
        match key {
            "byr" => self.byr = val,
            "iyr" => self.iyr = val,
            "eyr" => self.eyr = val,
            "hgt" => self.hgt = val,
            "hcl" => self.hcl = val,
            "ecl" => self.ecl = val,
            "pid" => self.pid = val,
            "cid" => self.cid = val,
            s => panic!("Unexpected key: {}", s),
        }
        self
    }

    fn required_fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        self.byr_is_valid()
            && self.iyr_is_valid()
            && self.eyr_is_valid()
            && self.hgt_is_valid()
            && self.hcl_is_valid()
            && self.ecl_is_valid()
            && self.pid_is_valid()
    }

    fn byr_is_valid(&self) -> bool {
        match &self.byr {
            Some(byr) => Passport::year_is_valid(byr, 1920, 2002),
            _ => false,
        }
    }

    fn iyr_is_valid(&self) -> bool {
        match &self.iyr {
            Some(iyr) => Passport::year_is_valid(iyr, 2010, 2020),
            _ => false,
        }
    }

    fn eyr_is_valid(&self) -> bool {
        match &self.eyr {
            Some(eyr) => Passport::year_is_valid(eyr, 2020, 2030),
            _ => false,
        }
    }

    fn hgt_is_valid(&self) -> bool {
        match &self.hgt {
            Some(hgt) => {
                if hgt.len() <= 2 {
                    false
                } else if &hgt[hgt.len() - 2..] == "cm" {
                    let height = hgt[..hgt.len() - 2].parse::<i32>();
                    match height {
                        Ok(height) => 150 <= height && height <= 193,
                        _ => false,
                    }
                } else if &hgt[hgt.len() - 2..] == "in" {
                    let height = hgt[..hgt.len() - 2].parse::<i32>();
                    match height {
                        Ok(height) => 59 <= height && height <= 76,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn hcl_is_valid(&self) -> bool {
        match &self.hcl {
            Some(hcl) => {
                if hcl.len() != 7 || !hcl.starts_with('#') {
                    false
                } else {
                    let legal_chars: HashSet<char> = [
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
                        'f',
                    ]
                    .iter()
                    .cloned()
                    .collect();
                    for chr in hcl[1..].chars() {
                        if !legal_chars.contains(&chr) {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }

    fn ecl_is_valid(&self) -> bool {
        match &self.ecl {
            Some(ecl) => {
                let legal_values: HashSet<&'static str> =
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .iter()
                        .cloned()
                        .collect();
                legal_values.contains(ecl.as_str())
            }
            _ => false,
        }
    }

    fn pid_is_valid(&self) -> bool {
        match &self.pid {
            Some(pid) => {
                if pid.len() != 9 {
                    false
                } else {
                    let legal_values: HashSet<char> =
                        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
                            .iter()
                            .cloned()
                            .collect();
                    for chr in pid.chars() {
                        if !legal_values.contains(&chr) {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }

    fn year_is_valid(s: &str, min: i32, max: i32) -> bool {
        if s.len() != 4 {
            return false;
        }
        let year = s.parse::<i32>();
        match year {
            Ok(year) => min <= year && year <= max,
            _ => false,
        }
    }
}

fn load_passports() -> Vec<Passport> {
    let contents = fs::read_to_string("data/day_4.txt").expect("File must be present");
    let mut state = ParserState::Key;
    let mut passports: Vec<Passport> = Vec::new();
    let mut buff = Vec::new();
    let mut key: String = String::new();
    let mut passport = Passport::new();
    for chr in contents.chars() {
        match &state {
            ParserState::Key => match chr {
                ':' => {
                    key = buff.iter().collect();
                    buff.clear();
                    state = ParserState::Val;
                }
                '\n' => {
                    passports.push(passport);
                    passport = Passport::new();
                }
                _ => {
                    buff.push(chr);
                }
            },
            ParserState::Val => match chr {
                ' ' | '\n' => {
                    let val: String = buff.iter().collect();
                    buff.clear();
                    passport = passport.with_key_val(&key, &val);
                    state = ParserState::Key;
                }
                _ => {
                    buff.push(chr);
                }
            },
        }
    }
    passports.push(passport);
    passports
}

fn main() {
    let passports = load_passports();
    let part_1_count = passports
        .iter()
        .filter(|p| p.required_fields_present())
        .count();
    let part_2_count = passports.iter().filter(|p| p.is_valid()).count();
    println!("{}", part_1_count);
    println!("{}", part_2_count);
}
