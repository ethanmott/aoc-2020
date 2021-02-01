use common::files;
use std::collections::HashMap;

fn main() {
    let lines = files::get_file_lines("input/day10.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    let mut joltages: Vec<i64> = lines.iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    joltages.sort();
    joltages.push(joltages[joltages.len() - 1] + 3);

    let mut one_jolt_diffs: Vec<i64> = Vec::new();
    let mut three_jolt_diffs: Vec<i64> = Vec::new();

    for (i, &j2) in joltages.iter().enumerate() {
        let mut j1 = 0;
        if i > 0 {
            j1 = joltages[i - 1];
        }

        if j2 - j1 == 1 {
            one_jolt_diffs.push(j1);
        } else if j2 - j1 == 3 {
            three_jolt_diffs.push(j1);
        }
    }

    one_jolt_diffs.len() as i64 * three_jolt_diffs.len() as i64
}

fn solve_part2(lines: &Vec<String>) -> i64 {
    let mut joltages: Vec<i64> = lines.iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    joltages.sort();
    let mut hm = HashMap::new();

    hm.insert(0, 1);

    joltages.iter()
        .for_each(|&joltage| {
            let ans = hm.get(&(joltage - 1)).unwrap_or(&0) +
                hm.get(&(joltage - 2)).unwrap_or(&0) +
                hm.get(&(joltage - 3)).unwrap_or(&0);
            hm.insert(joltage, ans);
        });

    hm[joltages.last().unwrap()]
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines1 = vec_of_strings![
            "16",
            "10",
            "15",
            "5",
            "1",
            "11",
            "7",
            "19",
            "6",
            "12",
            "4",
        ];
        let lines2 = vec_of_strings![
            "28",
            "33",
            "18",
            "42",
            "31",
            "14",
            "46",
            "20",
            "48",
            "47",
            "24",
            "23",
            "49",
            "45",
            "19",
            "38",
            "39",
            "11",
            "1",
            "32",
            "25",
            "35",
            "8",
            "17",
            "7",
            "9",
            "4",
            "2",
            "34",
            "10",
            "3",
        ];

        assert_eq!(solve_part1(&lines1), 35);
        assert_eq!(solve_part1(&lines2), 220);
    }

    #[test]
    fn test_solve_part2() {
        let lines1 = vec_of_strings![
            "16",
            "10",
            "15",
            "5",
            "1",
            "11",
            "7",
            "19",
            "6",
            "12",
            "4",
        ];
        let lines2 = vec_of_strings![
            "28",
            "33",
            "18",
            "42",
            "31",
            "14",
            "46",
            "20",
            "48",
            "47",
            "24",
            "23",
            "49",
            "45",
            "19",
            "38",
            "39",
            "11",
            "1",
            "32",
            "25",
            "35",
            "8",
            "17",
            "7",
            "9",
            "4",
            "2",
            "34",
            "10",
            "3",
        ];

        assert_eq!(solve_part2(&lines1), 8);
        assert_eq!(solve_part2(&lines2), 19208);
    }
}