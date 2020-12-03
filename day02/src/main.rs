use common::files;
use regex::Regex;

fn main() {
    let lines = files::get_file_lines("input/day02.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> usize {
    lines.iter()
        .map(|l| PasswordPolicyPair::<PasswordPolicyPart1>::from_line(l))
        .filter(|p| p.1.is_password_valid(p.0.clone()))
        .count()
}

fn solve_part2(lines: &Vec<String>) -> usize {
    lines.iter()
        .map(|l| PasswordPolicyPair::<PasswordPolicyPart2>::from_line(l))
        .filter(|p| p.1.is_password_valid(p.0.clone()))
        .count()
}

struct PasswordPolicyPair<T: PasswordPolicy>(String, T);

trait PasswordPolicy {
    fn is_password_valid(&self, password: String) -> bool;
}

#[derive(Debug)]
struct PasswordPolicyPart1 {
    min_letter_instances: usize,
    max_letter_instances: usize,
    letter: char,
}

impl PasswordPolicy for PasswordPolicyPart1 {
    fn is_password_valid(&self, password: String) -> bool {
        let num_letter_instances = password.matches(self.letter).count();

        num_letter_instances >= self.min_letter_instances && num_letter_instances <= self.max_letter_instances
    }
}

impl PasswordPolicyPair<PasswordPolicyPart1> {
    fn from_line(line: &String) -> Self {
        let re = Regex::new(r"^(\d+)-(\d+)\s([a-z]):\s([a-z]+)$").unwrap();

        let caps = re.captures(line).unwrap();
        let min_letter_instances = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max_letter_instances = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let password = caps.get(4).unwrap().as_str();

        Self(String::from(password), PasswordPolicyPart1 {
            min_letter_instances,
            max_letter_instances,
            letter,
        })
    }
}

#[derive(Debug)]
struct PasswordPolicyPart2 {
    letter_position_1: usize,
    letter_position_2: usize,
    letter: char,
}

impl PasswordPolicy for PasswordPolicyPart2 {
    fn is_password_valid(&self, password: String) -> bool {
        let c1 = password.chars().nth(self.letter_position_1 - 1).unwrap();
        let c2 = password.chars().nth(self.letter_position_2 - 1).unwrap();

        (c1 == self.letter && c2 != self.letter) || (c2 == self.letter && c1 != self.letter)
    }
}

impl PasswordPolicyPair<PasswordPolicyPart2> {
    fn from_line(line: &String) -> Self {
        let re = Regex::new(r"^(\d+)-(\d+)\s([a-z]):\s([a-z]+)$").unwrap();

        let caps = re.captures(line).unwrap();
        let letter_position_1 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let letter_position_2 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let password = caps.get(4).unwrap().as_str();

        Self(String::from(password), PasswordPolicyPart2 {
            letter_position_1,
            letter_position_2,
            letter,
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
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ];

        assert_eq!(solve_part1(&lines), 2);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec_of_strings![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ];

        assert_eq!(solve_part2(&lines), 1);
    }
}
