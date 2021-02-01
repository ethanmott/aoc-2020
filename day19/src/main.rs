#![feature(str_split_once)]

use common::files;
use std::collections::HashMap;

fn main() {
    let lines = files::get_file_lines("input/day19.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> usize {
    let rules = build_rules(&lines);

    lines.iter()
        .skip(rules.len() + 1)
        .map(|message| matches(message, &rules, 0))
        .filter(|results| results.iter().any(|&result| result.is_empty()))
        .count()
}

fn solve_part2(lines: &Vec<String>) -> usize {
    let mut rules = build_rules(&lines);

    rules.insert(8, Rule::Or((vec![42], vec![42, 8])));
    rules.insert(11, Rule::Or((vec![42, 31], vec![42, 11, 31])));

    lines.iter()
        .skip(rules.len() + 1)
        .map(|message| matches(message, &rules, 0))
        .filter(|results| results.iter().any(|&result| result.is_empty()))
        .count()
}

#[derive(Clone, Debug, PartialEq)]
enum Rule {
    Char(char),
    Rules(Vec<u64>),
    Or((Vec<u64>, Vec<u64>)),
}

fn build_rules(lines: &Vec<String>) -> HashMap<u64, Rule> {
    lines.iter()
        .take_while(|&line| !line.trim().is_empty())
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();

            let rule_number = parts[0].parse::<u64>().unwrap();
            let rule = match parts[1].trim().starts_with('"') {
                true => Rule::Char(parts[1].trim().trim_matches('"').chars().next().unwrap()),
                false => {
                    if parts[1].contains("|") {
                        let (left, right) = parts[1].split_once(" | ").unwrap();

                        Rule::Or((
                            left.split(" ").map(|n| n.parse::<u64>().unwrap()).collect(),
                            right.split(" ").map(|n| n.parse::<u64>().unwrap()).collect(),
                        ))
                    } else {
                        Rule::Rules(parts[1].split(" ").map(|n| n.parse::<u64>().unwrap()).collect())
                    }
                }
            };

            (rule_number, rule)
        })
        .collect::<HashMap<_, _>>()
}

fn matches<'a>(msg: &'a str, rules: &HashMap<u64, Rule>, rule_number: u64) -> Vec<&'a str> {
    match rules.get(&rule_number).unwrap() {
        Rule::Char(c) => {
            match msg.chars().next() {
                Some(ch) => {
                    if *c == ch {
                        vec![&msg[1..]]
                    } else {
                        vec![]
                    }
                }
                None => vec![]
            }
        }
        Rule::Rules(inner_rules) => {
            let mut remaining = vec![msg];

            for &r in inner_rules.iter() {
                let mut next = vec![];
                let mut no_match = true;

                for &s in remaining.iter() {
                    let mut results = matches(s, rules, r);
                    if !results.is_empty() {
                        no_match = false;
                    }
                    next.append(&mut results);
                }
                if no_match {
                    return vec![];
                }

                remaining = next;
            }

            remaining
        }
        Rule::Or((left, right)) => {
            let mut left_remaining = vec![msg];
            for &r in left.iter() {
                let mut next = vec![];
                let mut no_match = true;

                for &s in left_remaining.iter() {
                    let mut results = matches(s, rules, r);
                    if !results.is_empty() {
                        no_match = false;
                    }
                    next.append(&mut results);
                }
                if no_match {
                    left_remaining = vec![];
                    break;
                }

                left_remaining = next;
            }

            let mut right_remaining = vec![msg];
            for &r in right.iter() {
                let mut next = vec![];
                let mut no_match = true;

                for &s in right_remaining.iter() {
                    let mut results = matches(s, rules, r);
                    if !results.is_empty() {
                        no_match = false;
                    }
                    next.append(&mut results);
                }
                if no_match {
                    right_remaining = vec![];
                    break;
                }

                right_remaining = next;
            }

            left_remaining.append(&mut right_remaining);
            left_remaining
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines = vec_of_strings![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ];

        assert_eq!(solve_part1(&lines), 2);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec_of_strings![
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];

        assert_eq!(solve_part2(&lines), 12);
    }
}