use common::files;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let lines = files::get_file_lines("input/day14.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    let instructions: Vec<Instruction> = lines.iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = String::new();

    instructions.iter()
        .for_each(|i| {
            match i {
                Instruction::SetMask(value) => {
                    mask = value.clone();
                }
                Instruction::SetMemory(address, value) => {
                    memory.insert(address.clone(), mask_value(value, &mask));
                }
            };
        });

    memory.iter()
        .map(|(_, val)| val)
        .sum()
}

fn solve_part2(lines: &Vec<String>) -> i64 {
    let instructions: Vec<Instruction> = lines.iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = String::new();

    instructions.iter()
        .for_each(|i| {
            match i {
                Instruction::SetMask(value) => {
                    mask = value.clone();
                }
                Instruction::SetMemory(address, value) => {
                    for a in decode_address(address, &mask) {
                        memory.insert(a, value.clone());
                    }
                }
            };
        });

    memory.iter()
        .map(|(_, val)| val)
        .sum()
}

fn mask_value(value: &i64, mask: &String) -> i64 {
    let mut result = value.clone();

    mask.chars().rev()
        .fold(1 as i64, |bit_worth, c| {
            match c {
                '0' => {
                    result &= i64::MAX - bit_worth;
                }
                '1' => {
                    result |= bit_worth;
                }
                _ => {}
            };

            bit_worth * 2
        });

    result
}

fn decode_address(address: &i64, mask: &String) -> Vec<i64> {
    let mut addresses = Vec::new();
    addresses.push(address.clone());

    mask.chars().rev()
        .fold(1 as i64, |bit_worth, c| {
            let mut next = Vec::new();

            for &addr in addresses.iter() {
                match c {
                    '0' => {
                        next.push(addr);
                    }
                    '1' => {
                        next.push(addr | bit_worth);
                    }
                    'X' => {
                        next.push(addr & (i64::MAX - bit_worth));
                        next.push(addr | bit_worth);
                    }
                    x => panic!(format!("Invalid mask character: {}", x))
                };
            }

            addresses = next;

            bit_worth * 2
        });

    addresses
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    SetMask(String),
    SetMemory(i64, i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" = ").collect();

        Ok(match split[0] {
            "mask" => Instruction::SetMask(split[1].to_string()),
            x => {
                let address = x[4..x.len() - 1].parse::<i64>().unwrap();
                let value = split[1].parse::<i64>().unwrap();

                Instruction::SetMemory(address, value)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines = vec_of_strings![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ];

        assert_eq!(solve_part1(&lines), 165);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec_of_strings![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ];

        assert_eq!(solve_part2(&lines), 208);
    }
}