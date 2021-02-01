use common::files;
use std::collections::HashMap;

fn main() {
    let lines = files::get_file_lines("input/day18.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    lines.iter()
        .map(|s| evaluate(&s, false))
        .sum()
}

fn solve_part2(lines: &Vec<String>) -> i64 {
    lines.iter()
        .map(|s| evaluate(&s, true))
        .sum()
}

fn evaluate(s: &String, part2: bool) -> i64 {
    let mut value_stack = Vec::new();
    let mut operator_stack = Vec::new();

    let mut precedence_map = HashMap::new();
    precedence_map.insert('+', 1);
    precedence_map.insert('*', 2);

    for c in s.chars() {
        match c {
            ' ' => {},
            '(' => operator_stack.push(c),
            ')' => {
                while operator_stack.last().is_some() && operator_stack.last().unwrap() != &'(' {
                    let operator = operator_stack.pop().unwrap();
                    let operand1 = value_stack.pop().unwrap();
                    let operand2 = value_stack.pop().unwrap();

                    value_stack.push(apply_operator(operator, operand1, operand2));
                }
                operator_stack.pop();
            },
            '+' | '*' => {
                while operator_stack.last().is_some() && operator_stack.last().unwrap() != &'(' {
                    if part2 {
                        if precedence_map.get(&c).unwrap() < precedence_map.get(operator_stack.last().unwrap()).unwrap() {
                            break;
                        }
                    }

                    let operator = operator_stack.pop().unwrap();
                    let operand1 = value_stack.pop().unwrap();
                    let operand2 = value_stack.pop().unwrap();

                    value_stack.push(apply_operator(operator, operand1, operand2));
                }
                operator_stack.push(c);
            },
            x => {
                value_stack.push(x.to_string().parse::<i64>().unwrap());
            },
        };
    }

    while operator_stack.last().is_some() {
        let operator = operator_stack.pop().unwrap();
        let operand1 = value_stack.pop().unwrap();
        let operand2 = value_stack.pop().unwrap();

        value_stack.push(apply_operator(operator, operand1, operand2));
    }

    value_stack.pop().unwrap()
}

fn apply_operator(operator: char, operand1: i64, operand2: i64) -> i64 {
    match operator {
        '+' => operand1 + operand2,
        '*' => operand1 * operand2,
        x => panic!(format!("Invalid operator: {}", x)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_part1() {
        let lines = vec_of_strings![
            "1 + 2 * 3 + 4 * 5 + 6",
            "1 + (2 * 3) + (4 * (5 + 6))",
        ];

        assert_eq!(solve_part1(&lines), 122);
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate(&"1".to_string(), false), 1);
        assert_eq!(evaluate(&"1 + 2 * 3 + 4 * 5 + 6".to_string(), false), 71);
        assert_eq!(evaluate(&"1 + (2 * 3) + (4 * (5 + 6))".to_string(), false), 51);
        assert_eq!(evaluate(&"2 * 3 + (4 * 5)".to_string(), false), 26);
        assert_eq!(evaluate(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(), false), 437);
        assert_eq!(evaluate(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(), false), 12240);
        assert_eq!(evaluate(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(), false), 13632);
    }

    #[test]
    fn test_evaluate_part2() {
        assert_eq!(evaluate(&"1".to_string(), true), 1);
        assert_eq!(evaluate(&"1 + 2 * 3 + 4 * 5 + 6".to_string(), true), 231);
        assert_eq!(evaluate(&"1 + (2 * 3) + (4 * (5 + 6))".to_string(), true), 51);
        assert_eq!(evaluate(&"2 * 3 + (4 * 5)".to_string(), true), 46);
        assert_eq!(evaluate(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(), true), 1445);
        assert_eq!(evaluate(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(), true), 669060);
        assert_eq!(evaluate(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(), true), 23340);
    }
}