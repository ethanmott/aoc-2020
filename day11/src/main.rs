use common::files;

static NEIGHBOR_DIRS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn main() {
    let lines = files::get_file_lines("input/day11.txt");

    println!("part1: {:?}", solve(&lines, false));
    println!("part2: {:?}", solve(&lines, true));
}

fn solve(lines: &Vec<String>, part2: bool) -> usize {
    let mut grid = create_grid(lines);
    let mut num_changed = -1;

    while num_changed != 0 {
        let (new_grid, changed) = match part2 {
            false => step_grid_part1(&grid),
            true => step_grid_part2(&grid),
        };
        grid = new_grid;
        num_changed = changed;
    }

    grid.iter()
        .flatten()
        .filter(|&pos| *pos == PositionType::SEAT(true))
        .count()
}

#[derive(Clone, Debug, PartialEq)]
enum PositionType {
    FLOOR,
    SEAT(bool),
}

type Grid = Vec<Vec<PositionType>>;

fn create_grid(lines: &Vec<String>) -> Grid {
    let grid_height = lines.len();
    let grid_width = lines.iter().next().unwrap().len();

    let mut grid = vec![vec![PositionType::FLOOR; grid_width]; grid_height];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let square_type = match c {
                '.' => PositionType::FLOOR,
                'L' => PositionType::SEAT(false),
                '#' => PositionType::SEAT(true),
                _ => panic!("Invalid grid character.")
            };

            grid[y][x] = square_type;
        }
    }

    grid
}

fn step_grid_part1(grid: &Grid) -> (Grid, i32) {
    let mut new_grid = grid.clone();
    let mut num_changed = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                PositionType::FLOOR => {}
                PositionType::SEAT(false) => {
                    if count_adjacent_seats(&grid, x as i32, y as i32) == 0 {
                        new_grid[y][x] = PositionType::SEAT(true);
                        num_changed += 1;
                    }
                }
                PositionType::SEAT(true) => {
                    if count_adjacent_seats(&grid, x as i32, y as i32) >= 4 {
                        new_grid[y][x] = PositionType::SEAT(false);
                        num_changed += 1
                    }
                }
            };
        }
    }

    (new_grid, num_changed)
}

fn step_grid_part2(grid: &Grid) -> (Grid, i32) {
    let mut new_grid = grid.clone();
    let mut num_changed = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                PositionType::FLOOR => {}
                PositionType::SEAT(false) => {
                    if count_line_of_sight_seats(&grid, x as i32, y as i32) == 0 {
                        new_grid[y][x] = PositionType::SEAT(true);
                        num_changed += 1;
                    }
                }
                PositionType::SEAT(true) => {
                    if count_line_of_sight_seats(&grid, x as i32, y as i32) >= 5 {
                        new_grid[y][x] = PositionType::SEAT(false);
                        num_changed += 1
                    }
                }
            };
        }
    }

    (new_grid, num_changed)
}

fn on_grid(grid: &Grid, x: i32, y: i32) -> bool {
    y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32
}

fn count_adjacent_seats(grid: &Grid, x: i32, y: i32) -> usize {
    NEIGHBOR_DIRS.iter()
        .filter_map(|(dx, dy)| grid.get((y + dy) as usize).and_then(|row| row.get((x + dx) as usize)))
        .filter(|&pos| pos == &PositionType::SEAT(true))
        .count()
}

fn count_line_of_sight_seats(grid: &Grid, x: i32, y: i32) -> usize {
    NEIGHBOR_DIRS.iter()
        .filter(|(dx, dy)| {
            let mut new_x = x + dx;
            let mut new_y = y + dy;

            while on_grid(grid, new_x, new_y) && grid[new_y as usize][new_x as usize] == PositionType::FLOOR {
                new_x += dx;
                new_y += dy;
            }

            on_grid(grid, new_x, new_y) && grid[new_y as usize][new_x as usize] == PositionType::SEAT(true)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines = vec_of_strings![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];

        assert_eq!(solve(&lines, false), 37);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec_of_strings![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];

        assert_eq!(solve(&lines, true), 26);
    }
}
