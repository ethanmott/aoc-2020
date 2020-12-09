use std::str::FromStr;
use common::files;

fn main() {
    let lines = files::get_file_lines("input/day08.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i32 {
    let instructions: Vec<Instruction> = lines.iter()
        .map(|s| s.as_str())
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    let mut executed_instructions: Vec<i32> = Vec::new();
    let mut pc = 0;
    let mut acc = 0;

    loop {
        let i = instructions[pc as usize];

        executed_instructions.push(pc);

        match i.operation {
            Operation::ACC => {
                acc += i.argument;
                pc += 1;
            },
            Operation::JMP => pc += i.argument,
            Operation::NOP => pc += 1,
        }

        if executed_instructions.contains(&pc) {
            return acc;
        }
    }
}

fn solve_part2(lines: &Vec<String>) -> i32 {
    let original_instructions: Vec<Instruction> = lines.iter()
        .map(|s| s.as_str())
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    for instructions in permutate_instructions(original_instructions) {
        let max_iterations = 99999;
        let mut num_executed = 0;
        let mut pc = 0;
        let mut acc = 0;

        let val = loop {
            let i = instructions[pc as usize];

            match i.operation {
                Operation::ACC => {
                    acc += i.argument;
                    pc += 1;
                },
                Operation::JMP => pc += i.argument,
                Operation::NOP => pc += 1,
            }

            num_executed += 1;

            if num_executed > max_iterations {
                break -1;
            } else if pc as usize == instructions.len() {
                break acc;
            }
        };

        if val != -1 {
            return acc;
        }
    }

    0
}

fn permutate_instructions(instructions: Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut permutations: Vec<Vec<Instruction>> = Vec::new();

    for i in 0..instructions.len() {
        let mut permutation = instructions.clone();

        permutation[i] = match instructions[i].operation {
            Operation::ACC => continue,
            Operation::JMP => Instruction {
                operation: Operation::NOP,
                argument: instructions[i].argument,
            },
            Operation::NOP => Instruction {
                operation: Operation::JMP,
                argument: instructions[i].argument,
            },
        };

        permutations.push(permutation);
    }

    permutations
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let operation = match split[0] {
            "acc" => Operation::ACC,
            "jmp" => Operation::JMP,
            "nop" => Operation::NOP,
            x => panic!("Invalid operation: {}", x)
        };
        let argument = match split[1].chars().next().unwrap() {
            '+' => split[1][1..].parse::<i32>().unwrap(),
            '-' => split[1][1..].parse::<i32>().unwrap() * -1,
            x => panic!("Invalid sign: {}", x)
        };

        Ok(Instruction {
            operation,
            argument,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let input = vec_of_strings![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ];

        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn test_solve_part2() {
        let input = vec_of_strings![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ];

        assert_eq!(solve_part2(&input), 8);
    }
}