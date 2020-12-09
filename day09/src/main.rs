use common::files;
use itertools::Itertools;

fn main() {
    let nums: Vec<i64> = files::get_file_lines("input/day09.txt").iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let part1_result =  solve_part1(&nums, 25);

    println!("part1: {:?}", part1_result);
    println!("part2: {:?}", solve_part2(&nums, part1_result.unwrap()));
}

fn solve_part1(nums: &Vec<i64>, preamble_length: usize) -> Option<i64> {
    let mut i = 0;
    for &num in nums.iter().skip(preamble_length) {
        let mut has_pair = false;

        for n in  nums.iter().skip(i).take(preamble_length).permutations(2).unique() {
            if n[0] + n[1] == num {
                has_pair = true;
            }
        }

        if !has_pair {
            return Some(num);
        }

        i += 1;
    }

    None
}

fn solve_part2(nums: &Vec<i64>, target: i64) -> Option<i64> {
    for (i, _) in nums.iter().enumerate() {
        let mut sum = 0;

        let mut set: Vec<&i64> = nums.iter().skip(i)
            .take_while(|&n| {
                if sum + n <= target {
                    sum += n;

                    return true;
                }

                false
            })
            .collect();

        if sum == target {
            set.sort();
            return Some(set[0] + set[set.len() - 1]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];

        assert_eq!(solve_part1(&input, 5), Some(127));
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];

        assert_eq!(solve_part2(&input, 127), Some(62));
    }
}