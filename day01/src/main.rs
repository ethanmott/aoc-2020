use common::files;

fn main() {
    let lines = files::get_file_lines("input/day01.txt");

    println!("part1: {:?}", solve_part1(&lines, 2020));
    println!("part2: {:?}", solve_part2(&lines, 2020));
}

fn solve_part1(lines: &Vec<String>, target: u32) -> Option<u32> {
    let nums: Vec<u32> = lines.iter()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    for (i1, n1) in nums.iter().enumerate() {
        for (i2, n2) in nums.iter().enumerate() {
            if i1 != i2 && n1 + n2 == target {
                return Some(n1 * n2)
            }
        }
    }

    None
}

fn solve_part2(lines: &Vec<String>, target: u32) -> Option<u32> {
    let nums: Vec<u32> = lines.iter()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    for (i1, n1) in nums.iter().enumerate() {
        for (i2, n2) in nums.iter().enumerate() {
            for (i3, n3) in nums.iter().enumerate() {
                if i1 != i2 && i1 != i3 && i2 != i3 && n1 + n2 + n3 == target {
                    return Some(n1 * n2 * n3)
                }
            }
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines = vec_of_strings![
            "1721",
            "979",
            "366",
            "299",
            "675",
            "1456",
        ];

        assert_eq!(solve_part1(&lines, 2020), Some(514579));
        assert_eq!(solve_part1(&lines, 2021), None);
        assert_eq!(solve_part1(&lines, 99999), None);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec_of_strings![
            "1721",
            "979",
            "366",
            "299",
            "675",
            "1456",
        ];

        assert_eq!(solve_part2(&lines, 2020), Some(241861950));
        assert_eq!(solve_part1(&lines, 2021), None);
        assert_eq!(solve_part1(&lines, 99999), None);
    }
}