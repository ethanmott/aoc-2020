use common::files;

const ROW_LOWER_BOUND: u32 = 0;
const ROW_UPPER_BOUND: u32 = 127;
const COLUMN_LOWER_BOUND: u32 = 0;
const COLUMN_UPPER_BOUND: u32 = 7;

fn main() {
    let lines = files::get_file_lines("input/day05.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> Option<u32> {
    lines.iter()
        .map(|l| decode_line(l))
        .max()
}

fn solve_part2(lines: &Vec<String>) -> Option<u32> {
    let mut seat_ids: Vec<u32> = lines.iter()
        .map(|l| decode_line(l))
        .collect();

    seat_ids.sort();

    for (i, seat_id) in seat_ids.iter().enumerate() {
        if i != 0 && seat_ids[i - 1] != seat_id - 1 {
            return Some(seat_id - 1)
        }
    }

    None
}

fn decode_line(line: &String) -> u32 {
    let mut row_low = ROW_LOWER_BOUND;
    let mut row_high = ROW_UPPER_BOUND;
    let mut col_low = COLUMN_LOWER_BOUND;
    let mut col_high = COLUMN_UPPER_BOUND;

    for c in line.chars() {
        match c {
            'F' => row_high = row_high - ((row_high - row_low) + 1) / 2,
            'B' => row_low = row_low + ((row_high - row_low) + 1) / 2,
            'L' => col_high = col_high - ((col_high - col_low) + 1) / 2,
            'R' => col_low = col_low + ((col_high - col_low) + 1) / 2,
            x => panic!("Invalid character: {}", x)
        };
    }

    (row_low * 8) + col_low
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let input = vec_of_strings![
            "FBFBBFFRLR",
            "BFFFBBFRRR",
            "FFFBBBFRRR",
            "BBFFBBFRLL",
        ];

        assert_eq!(solve_part1(&input), Some(820));
    }

    #[test]
    fn test_solve_part2() {
        let input = vec_of_strings![
            "FFFFFFFLLL", // 0
            "FFFFFFFLLR", // 1
            "FFFFFFFLRL", // 2
            "FFFFFFFRLL", // 4
        ];

        assert_eq!(solve_part2(&input), Some(3));
    }

    #[test]
    fn test_decode_line() {
        assert_eq!(decode_line(&"FBFBBFFRLR".to_string()), 357);
        assert_eq!(decode_line(&"BFFFBBFRRR".to_string()), 567);
        assert_eq!(decode_line(&"FFFBBBFRRR".to_string()), 119);
        assert_eq!(decode_line(&"BBFFBBFRLL".to_string()), 820);
    }
}