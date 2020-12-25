use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Copy, Clone)]
struct Instruction(Operation, i64);

fn parse_instructions(input: &str) -> Vec<Instruction> {
    peg::parser! {
        pub(crate) grammar parser() for str {
            pub(crate) rule root() -> Vec<Instruction>
                = i:instruction()+ ![_] { i }

            rule instruction() -> Instruction
                = o:operation() " " a:argument() newline()? { Instruction(o, a) }

            rule operation() -> Operation
                = "acc" { Operation::Acc }
                / "jmp" { Operation::Jmp }
                / "nop" { Operation::Nop }

            rule argument() -> i64
                = s:sign() n:number() { s * n }

            rule sign() -> i64
                = "+" { 1 }
                / "-" { -1 }

            rule number() -> i64
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule newline()
                = ['\n' | '\r']+
        }
    }
    parser::root(input).unwrap()
}

#[derive(Default, Debug)]
struct State {
    accumulator: i64,
    program_counter: i64,
}

impl State {
    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction(Operation::Acc, arg) => {
                self.accumulator += arg;
                self.program_counter += 1;
            }
            Instruction(Operation::Jmp, arg) => {
                self.program_counter += arg;
            }
            Instruction(Operation::Nop, _) => {
                self.program_counter += 1;
            }
        }
    }
}

fn part1(instructions: &[Instruction]) -> i64 {
    let mut executed_instructions: HashSet<i64> = Default::default();
    let mut state: State = Default::default();

    loop {
        let instruction = &instructions[state.program_counter as usize];
        if executed_instructions.contains(&state.program_counter) {
            break;
        }
        executed_instructions.insert(state.program_counter);
        state.execute_instruction(&instruction);
    }
    state.accumulator
}

fn part2(instructions: &[Instruction]) -> i64 {
    let fixable_instructions: Vec<(usize, &Instruction)> = instructions
        .iter()
        .enumerate()
        .filter(|(_idx, Instruction(op, _arg))| match op {
            Operation::Jmp | Operation::Nop => true,
            Operation::Acc => false,
        })
        .collect();

    for (fixable_instruction_index, fixable_instruction) in fixable_instructions {
        let mut executed_instructions: HashSet<i64> = Default::default();
        let mut state: State = Default::default();

        loop {
            if executed_instructions.contains(&state.program_counter) {
                break;
            }

            if state.program_counter as usize == instructions.len() {
                return state.accumulator;
            }

            let instruction = if state.program_counter as usize == fixable_instruction_index {
                match fixable_instruction {
                    Instruction(Operation::Jmp, arg) => Instruction(Operation::Nop, *arg),
                    Instruction(Operation::Nop, arg) => Instruction(Operation::Jmp, *arg),
                    Instruction(Operation::Acc, _) => panic!("Acc instructions can't be fixed"),
                }
            } else {
                instructions[state.program_counter as usize]
            };

            executed_instructions.insert(state.program_counter);
            state.execute_instruction(&instruction);
        }
    }

    panic!("No solution found");
}

fn main() {
    let input = include_str!("../../data/day_8.txt");
    let instructions = parse_instructions(&input);
    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}
