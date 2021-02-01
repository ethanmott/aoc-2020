use common::files;
use std::str::FromStr;

fn main() {
    let lines = files::get_file_lines("input/day12.txt");

    println!("part1: {:?}", solve_part1(&lines));
    println!("part2: {:?}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i32 {
    let actions = lines.iter()
        .map(|l| Action::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let mut ship = Ship {
        x: 0,
        y: 0,
        facing: Direction::E,
    };

    actions.iter()
        .for_each(|a| {
            match a {
                Action::NORTH(val) => ship.y += val,
                Action::SOUTH(val) => ship.y -= val,
                Action::EAST(val) => ship.x += val,
                Action::WEST(val) => ship.x -= val,
                Action::LEFT(_) => ship.facing = ship.facing.apply_rotation(a),
                Action::RIGHT(_) => ship.facing = ship.facing.apply_rotation(a),
                Action::FORWARD(val) => {
                    match ship.facing {
                        Direction::N => ship.y += val,
                        Direction::S => ship.y -= val,
                        Direction::E => ship.x += val,
                        Direction::W => ship.x -= val,
                    };
                }
            }
        });

    ship.x.abs() + ship.y.abs()
}

fn solve_part2(lines: &Vec<String>) -> i32 {
    let actions = lines.iter()
        .map(|l| Action::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let mut ship = Ship {
        x: 0,
        y: 0,
        facing: Direction::E,
    };
    let mut waypoint = Waypoint {
        x: 10,
        y: 1,
    };

    actions.iter()
        .for_each(|a| {
            match a {
                Action::NORTH(val) => waypoint.y += val,
                Action::SOUTH(val) => waypoint.y -= val,
                Action::EAST(val) => waypoint.x += val,
                Action::WEST(val) => waypoint.x -= val,
                Action::LEFT(_) => waypoint = waypoint.apply_rotation(&a),
                Action::RIGHT(_) => waypoint = waypoint.apply_rotation(&a),
                Action::FORWARD(val) => {
                    ship.x += waypoint.x * val;
                    ship.y += waypoint.y * val;
                }
            }
        });

    ship.x.abs() + ship.y.abs()
}

#[derive(Clone, Debug, PartialEq)]
enum Action {
    NORTH(i32),
    EAST(i32),
    SOUTH(i32),
    WEST(i32),
    LEFT(i32),
    RIGHT(i32),
    FORWARD(i32),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s[1..].trim().parse::<i32>().unwrap();

        Ok(match &s[..1] {
            "N" => Action::NORTH(val),
            "E" => Action::EAST(val),
            "S" => Action::SOUTH(val),
            "W" => Action::WEST(val),
            "L" => Action::LEFT(val),
            "R" => Action::RIGHT(val),
            "F" => Action::FORWARD(val),
            _ => panic!(format!("Invalid action: {}", s)),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn apply_rotation(&self, action: &Action) -> Direction {
        let degrees = match *action {
            Action::LEFT(val) => 360 - val,
            Action::RIGHT(val) => val,
            _ => 0,
        };

        match self {
            Direction::N => {
                match degrees {
                    90 => Direction::E,
                    180 => Direction::S,
                    270 => Direction::W,
                    _ => Direction::N
                }
            }
            Direction::E => {
                match degrees {
                    90 => Direction::S,
                    180 => Direction::W,
                    270 => Direction::N,
                    _ => Direction::E
                }
            }
            Direction::S => {
                match degrees {
                    90 => Direction::W,
                    180 => Direction::N,
                    270 => Direction::E,
                    _ => Direction::S
                }
            }
            Direction::W => {
                match degrees {
                    90 => Direction::N,
                    180 => Direction::E,
                    270 => Direction::S,
                    _ => Direction::W
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Ship {
    x: i32,
    y: i32,
    facing: Direction,
}

#[derive(Clone, Debug, PartialEq)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn apply_rotation(&self, action: &Action) -> Waypoint {
        let degrees = match *action {
            Action::LEFT(val) => 360 - val,
            Action::RIGHT(val) => val,
            _ => 0,
        };

        match degrees {
            90 => Waypoint {
                x: self.y,
                y: -self.x,
            },
            180 => Waypoint {
                x: -self.x,
                y: -self.y,
            },
            270 => Waypoint {
                x: -self.y,
                y: self.x,
            },
            _ => Waypoint {
                x: self.x,
                y: self.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve_part1() {
        let lines = vec_of_strings![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11",
        ];

        assert_eq!(solve_part1(&lines), 25);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec_of_strings![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11",
        ];

        assert_eq!(solve_part2(&lines), 286);
    }
}
