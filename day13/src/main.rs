use common::files;

fn main() {
    let lines = files::get_file_lines("input/day13.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    let arrival_timestamp = lines[0].parse::<i64>().unwrap();
    let bus_ids: Vec<i64> = lines[1].split(",")
        .filter(|&s| s != "x")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut closest_bus_id = -1;
    let mut closest = i64::MAX;
    for bus_id in bus_ids {
        if bus_id - (arrival_timestamp % bus_id) < closest {
            closest_bus_id = bus_id;
            closest = bus_id - (arrival_timestamp % bus_id);
        }
    }

    closest * closest_bus_id
}

fn solve_part2(lines: &Vec<String>) -> Option<i64> {
    let mut modulii = Vec::new();
    let mut residues = Vec::new();
    lines[1].split(",")
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .for_each(|(i, s)| {
            let val = s.parse::<i64>().unwrap();

            modulii.push(val);
            residues.push(val - i as i64);
        });

    chinese_remainder(&residues, &modulii)
}

// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec_of_strings!["939", "7,13,x,x,59,x,31,19"]), 295);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec_of_strings!["939", "17,x,13,19"]), Some(3417));
        assert_eq!(solve_part2(&vec_of_strings!["939", "67,7,59,61"]), Some(754018));
        assert_eq!(solve_part2(&vec_of_strings!["939", "67,x,7,59,61"]), Some(779210));
        assert_eq!(solve_part2(&vec_of_strings!["939", "67,7,x,59,61"]), Some(1261476));
        assert_eq!(solve_part2(&vec_of_strings!["939", "1789,37,47,1889"]), Some(1202161486));
    }
}