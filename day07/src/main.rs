use common::files;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let lines = files::get_file_lines("input/day07.txt");

    println!("part1: {:?}", solve_part1(&lines, "shiny gold"));
    println!("part2: {:?}", solve_part2(&lines, "shiny gold"));
}

fn solve_part1(lines: &Vec<String>, target_color: &str) -> i32 {
    let map = build_map(lines);

    map.iter()
        .map(|(color, _)| match can_contain(&map, color, target_color) {
            true => 1,
            false => 0,
        })
        .sum()
}

fn solve_part2(lines: &Vec<String>, target_color: &str) -> i32 {
    let map = build_map(lines);

   total_bags(&map, target_color) - 1
}

fn can_contain(map: &HashMap<String, Vec<(String, i32)>>, color: &String, target_color: &str) -> bool {
    map.get(color).iter()
        .any(|&contains| {
            contains.iter()
                .any(|(color, _)| color == target_color || can_contain(map, color, target_color))
        })
}

fn total_bags(map: &HashMap<String, Vec<(String, i32)>>, target_color: &str) -> i32 {
    map.get(target_color).iter()
        .fold(1, |acc, &contains| {
            let sum: i32 = contains.iter()
                .map(|(color, quantity)| quantity * total_bags(map, color))
                .sum();

            acc + sum
        })
}

fn build_map(lines: &Vec<String>) -> HashMap<String, Vec<(String, i32)>> {
    let mut map: HashMap<String, Vec<(String, i32)>> = HashMap::new();

    for l in lines {
        let bag_color = Regex::new(r"^(.+) bags contain").unwrap()
            .captures(l).unwrap()[1].to_string();
        let contains = Regex::new("([0-9]+) ([a-z\\s]+) bag").unwrap()
            .captures_iter(l)
            .map(|caps| {
                (caps[2].to_string(), caps[1].parse::<i32>().unwrap())
            })
            .collect();

        map.insert(bag_color, contains);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let input = vec_of_strings![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        assert_eq!(solve_part1(&input, "shiny gold"), 4);
    }

    #[test]
    fn test_solve_part2() {
        let input = vec_of_strings![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        assert_eq!(solve_part2(&input, "shiny gold"), 126);
    }
}

