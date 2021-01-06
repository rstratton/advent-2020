use std::collections::HashMap;

#[derive(Copy, Clone)]
enum BitSpec {
    X,
    Zero,
    One,
}

#[derive(Copy, Clone)]
struct Mask {
    bits: [BitSpec; 36],
}

impl Mask {
    fn apply_part_1(&self, val: u64) -> u64 {
        let mut result = val;

        for (idx, bit_spec) in self.bits.iter().rev().enumerate() {
            match bit_spec {
                BitSpec::X => {}
                BitSpec::Zero => result &= !0 ^ (1 << idx),
                BitSpec::One => result |= 1 << idx,
            }
        }

        result
    }

    fn apply_part_2(&self, val: u64) -> Vec<u64> {
        let mut masked = [BitSpec::X; 36];

        let bit_values = (0..36)
            .into_iter()
            .rev()
            .map(|bit_idx| (val >> bit_idx) & 1);
        let bit_specs = self.bits.iter();

        for (idx, (bit_value, bit_spec)) in bit_values.zip(bit_specs).enumerate() {
            let masked_value = match bit_spec {
                BitSpec::X => BitSpec::X,
                BitSpec::Zero if bit_value == 0 => BitSpec::Zero,
                BitSpec::Zero => BitSpec::One,
                BitSpec::One => BitSpec::One,
            };
            masked[idx] = masked_value;
        }

        Mask::generate_variations(&masked)
    }

    fn generate_variations(bit_specs: &[BitSpec; 36]) -> Vec<u64> {
        let num_xs = bit_specs
            .iter()
            .filter(|bit_spec| matches!(bit_spec, BitSpec::X))
            .count();
        let num_results = 2usize.pow(num_xs as u32);
        let mut results = vec![0u64; num_results];

        let mut x_idx = 0;

        for (bit_idx, bit_spec) in bit_specs.iter().rev().enumerate() {
            match bit_spec {
                BitSpec::Zero => {}
                BitSpec::One => {
                    let bit = 1u64 << bit_idx;
                    for result in results.iter_mut() {
                        *result |= bit;
                    }
                }
                BitSpec::X => {
                    for (i, result) in results.iter_mut().enumerate() {
                        let bit = ((i >> x_idx) & 1) << bit_idx;
                        *result |= bit as u64;
                    }
                    x_idx += 1;
                }
            }
        }

        results
    }
}

impl From<&str> for Mask {
    fn from(s: &str) -> Self {
        let mut result = Self {
            bits: [BitSpec::X; 36],
        };

        for (idx, chr) in s.chars().enumerate() {
            match chr {
                'X' => {}
                '0' => result.bits[idx] = BitSpec::Zero,
                '1' => result.bits[idx] = BitSpec::One,
                _ => panic!("Unexpected character in mask {}", chr),
            }
        }

        result
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bit_string = String::new();
        for bit_spec in self.bits.iter() {
            bit_string.push(match bit_spec {
                BitSpec::X => 'X',
                BitSpec::Zero => '0',
                BitSpec::One => '1',
            })
        }
        f.debug_struct("Mask").field("bits", &bit_string).finish()
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    WriteMem(u64, u64),
}

#[derive(Default)]
struct State {
    mask: Option<Mask>,
    mem: HashMap<u64, u64>,
}

impl State {
    fn execute_part_1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(m) => self.mask = Some(*m),
            Instruction::WriteMem(addr, val) => {
                self.mem
                    .insert(*addr, self.mask.unwrap().apply_part_1(*val));
            }
        }
    }

    fn execute_part_2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(m) => self.mask = Some(*m),
            Instruction::WriteMem(addr, val) => {
                for addr_instance in self.mask.unwrap().apply_part_2(*addr) {
                    self.mem.insert(addr_instance, *val);
                }
            }
        }
    }

    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

fn parse_program(input: &str) -> Vec<Instruction> {
    peg::parser! {
        pub(crate) grammar parser() for str {
            pub(crate) rule root() -> Vec<Instruction>
                = i:instruction()+ ![_] { i }

            rule instruction() -> Instruction
                = m:mask() { m }
                / w:mem_write() { w }

            rule mask() -> Instruction
                = "mask = " m:$(['X' | '0' | '1']+) newline()? { Instruction::Mask(m.into()) }

            rule mem_write() -> Instruction
                = "mem[" a:number() "] = " v:number() newline()? { Instruction::WriteMem(a, v) }

            rule number() -> u64
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule newline()
                = ['\n' | '\r']+
        }
    }
    parser::root(input).unwrap()
}

fn part1(program: &[Instruction]) -> u64 {
    let mut state = State::default();
    for instruction in program {
        state.execute_part_1(instruction);
    }
    state.sum()
}

fn part2(program: &[Instruction]) -> u64 {
    let mut state = State::default();
    for instruction in program {
        state.execute_part_2(instruction);
    }
    state.sum()
}

fn main() {
    let input = include_str!("../../data/day_14.txt");
    let program = parse_program(input);

    println!("{}", part1(&program));
    println!("{}", part2(&program));
}
