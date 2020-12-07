use common::files;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref GROUP_SEPARATOR: Regex = Regex::new(r"\n\n|\r\n\r\n").unwrap();
}

fn main() {
    let input = files::get_file_as_string("input/day06.txt");

    println!("part1: {:?}", solve_part1(&input));
    println!("part2: {:?}", solve_part2(&input));
}

fn solve_part1(input: &String) -> usize {
    GROUP_SEPARATOR.split(input.trim())
        .map(String::from)
        .map(|mut s| {
            s.retain(|c| !c.is_whitespace());

            s
        })
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|mut chars| {
            chars.sort();
            chars.dedup();

            chars.len()
        })
        .sum()
}

fn solve_part2(input: &String) -> usize {
    let mut total_yes = 0;

    for group in GROUP_SEPARATOR.split(input.trim()) {
        let num_people = group.lines().count();
        let mut map: HashMap<char, usize> = HashMap::new();

        for person_answers in group.lines() {
            for c in person_answers.chars() {
                *map.entry(c).or_insert(0) += 1;
            }

            for entry in map.keys() {
                if *map.get(entry).unwrap() == num_people {
                    total_yes += 1
                }
            }
        }
    }

    total_yes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = r#"
            abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b
        "#.to_string();

        assert_eq!(solve_part1(&input), 11);
    }

    #[test]
    fn test_solve_part2() {
        let input = r#"
            abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b
        "#.to_string();

        assert_eq!(solve_part2(&input), 6);
    }
}