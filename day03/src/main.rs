use common::files;

fn main() {
    let lines = files::get_file_lines("input/day03.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    let grid = create_grid(lines);

    traverse_grid(&grid, 3, 1)
}

fn solve_part2(lines: &Vec<String>) -> i64 {
    let grid = create_grid(lines);

    traverse_grid(&grid, 1, 1)
        * traverse_grid(&grid, 3, 1)
        * traverse_grid(&grid, 5, 1)
        * traverse_grid(&grid, 7, 1)
        * traverse_grid(&grid, 1, 2)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SquareType {
    OPEN,
    TREE,
}

type Grid = Vec<Vec<SquareType>>;

fn create_grid(lines: &Vec<String>) -> Grid {
    let grid_height = lines.len();
    let grid_width = lines.iter().next().unwrap().len();

    let mut grid = vec![vec![SquareType::OPEN; grid_width]; grid_height];

    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let square_type = match c {
                '.' => SquareType::OPEN,
                '#' => SquareType::TREE,
                _ => panic!("Invalid grid character.")
            };

            grid[x][y] = square_type;
        }
    }

    grid
}

fn traverse_grid(grid: &Grid, dx: usize, dy: usize) -> i64 {
    let grid_height = grid.len();
    let grid_width = grid.iter().next().unwrap().len();

    let mut trees_encountered = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < grid_height {
        pos_x = (pos_x + dx) % grid_width;
        pos_y += dy;

        if pos_y < grid_height && grid[pos_y][pos_x] == SquareType::TREE {
            trees_encountered += 1;
        }

    }

    trees_encountered
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines = vec_of_strings![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        assert_eq!(solve_part1(&lines), 7);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec_of_strings![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        assert_eq!(solve_part2(&lines), 336);
    }
}
