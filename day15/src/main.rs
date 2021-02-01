use std::collections::HashMap;

fn main() {
    let starting_numbers = vec![11, 0, 1, 10, 5, 19];

    println!("part1: {:?}", solve(starting_numbers.clone(), 2020));
    println!("part2: {:?}", solve(starting_numbers.clone(), 30000000));
}

fn solve(starting_numbers: Vec<i32>, num_turns: i32) -> i32 {
    let mut seen: HashMap<i32, i32> = HashMap::new();
    let mut seen2: HashMap<i32, i32> = HashMap::new();
    let mut last = starting_numbers.last().unwrap().clone();

    for (i, num) in starting_numbers.iter().enumerate() {
        seen.insert(num.clone(), i as i32 + 1);
    }

    for turn_number in starting_numbers.len() as i32 + 1..=num_turns {
        let mut next = 0;
        if seen.contains_key(&last) {
            let last_turn_number_seen = seen.get(&last).unwrap();

            if seen2.contains_key(&last) {
                next = last_turn_number_seen - seen2.get(&last).unwrap();
            }
        }

        if seen.contains_key(&next) {
            seen2.insert(next, seen.get(&next).unwrap().clone());
        }
        seen.insert(next, turn_number);

        last = next.clone();
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(vec![0, 3, 6], 10), 0);
        assert_eq!(solve(vec![0, 3, 6], 2020), 436);
    }
}
