use common::files;
use std::ops::RangeInclusive;
use std::collections::HashSet;

fn main() {
    let lines = files::get_file_lines("input/day16.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

type Rule = Vec<RangeInclusive<usize>>;
type Ticket = Vec<usize>;

fn rule_from_str(s: &str) -> Rule {
    let split: Vec<&str> = s.split(": ").collect();

    split[1].split(" or ")
        .map(|s| {
            let nums = s.split("-")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            RangeInclusive::new(nums[0], nums[1])
        })
        .collect::<Vec<_>>()
}

fn ticket_from_str(s: &str) -> Ticket {
    s.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()
}

fn satisfies_rule(rule: &Rule, value: usize) -> bool {
    rule.iter().any(|range| range.contains(&value))
}

fn solve_part1(lines: &Vec<String>) -> usize {
    let rules = lines.iter()
        .take_while(|&s| s.trim() != "")
        .map(|s| rule_from_str(s))
        .collect::<Vec<_>>();

    let nearby_tickets = lines.iter()
        .skip_while(|&s| s != "nearby tickets:")
        .skip(1)
        .map(|s| ticket_from_str(s))
        .collect::<Vec<_>>();

    let mut result = 0;
    nearby_tickets.iter()
        .for_each(|t| {
            match t.iter().find(|&&val| rules.iter().all(|r| !satisfies_rule(r, val))) {
                Some(v) => result += v,
                None => {}
            }
        });

    result
}

fn possible_rules(rules: &Vec<Rule>, tickets: &Vec<Ticket>) -> Vec<HashSet<usize>> {
    (0..rules.len()).map(|i|
        (0..rules.len()).filter(|&j|
            tickets.iter().all(|t|  satisfies_rule(&rules[j], t[i]))
        ).collect()
    ).collect()
}

fn solve_part2(lines: &Vec<String>) -> usize {
    let rules = lines.iter()
        .take_while(|&s| s.trim() != "")
        .map(|s| rule_from_str(s))
        .collect::<Vec<_>>();

    let my_ticket = lines.iter()
        .skip_while(|&s| s != "your ticket:")
        .skip(1)
        .next()
        .map(|s| ticket_from_str(s))
        .unwrap();

    let nearby_tickets = lines.iter()
        .skip_while(|&s| s != "nearby tickets:")
        .skip(1)
        .map(|s| ticket_from_str(s))
        .collect::<Vec<_>>();

    let mut valid_tickets = Vec::new();
    valid_tickets.push(my_ticket.clone());

    nearby_tickets.iter()
        .for_each(|t| {
            match t.iter().find(|&&val| rules.iter().all(|r| !satisfies_rule(r, val))) {
                Some(_) => {}
                None => valid_tickets.push(t.clone()),
            }
        });

    let mut possible_rules = possible_rules(&rules, &valid_tickets);
    let mut assigned_rules = [0; 20];
    while let Some(i) = possible_rules.iter().position(|s| s.len() == 1) {
        let v = *possible_rules[i].iter().next().unwrap();
        assigned_rules[i] = v;
        for s in &mut possible_rules {
            s.remove(&v);
        }
    }
    assigned_rules.iter()
        .enumerate()
        .filter(|(_,&rule)| rule < 6)
        .map(|(i,_)| my_ticket[i])
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines = vec_of_strings![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ];

        assert_eq!(solve_part1(&lines), 71);
    }
}