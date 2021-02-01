use common::files;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let lines = files::get_file_lines("input/day17.txt");

    let (part1, part2) = solve(&lines);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn solve(lines: &Vec<String>) -> (usize, usize) {
    let mut active_coords = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_coords.insert((x as i64, y as i64));
            }
        }
    }

    (
        simulate(active_coords.iter().map(|&(x, y)| Coord3D(x, y, 0)).collect()),
        simulate(active_coords.iter().map(|&(x, y)| Coord4D(x, y, 0, 0)).collect()),
    )
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord3D(i64, i64, i64);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord4D(i64, i64, i64, i64);

trait Adjacents
    where Self: Copy + Eq + Hash + Sized {
    fn get_adjacent(&self) -> Vec<Self>;
}

impl Adjacents for Coord3D {
    fn get_adjacent(&self) -> Vec<Self> {
        let mut result = Vec::new();
        let (x, y, z) = (self.0, self.1, self.2);

        for new_x in x - 1..=x + 1 {
            for new_y in y - 1..=y + 1 {
                for new_z in z - 1..=z + 1 {
                    if new_x != x || new_y != y || new_z != z {
                        result.push(Coord3D(new_x, new_y, new_z));
                    }
                }
            }
        }

        result
    }
}

impl Adjacents for Coord4D {
    fn get_adjacent(&self) -> Vec<Self> {
        let mut result = Vec::new();
        let (x, y, z, w) = (self.0, self.1, self.2, self.3);

        for new_x in x - 1..=x + 1 {
            for new_y in y - 1..=y + 1 {
                for new_z in z - 1..=z + 1 {
                    for new_w in w - 1..=w + 1 {
                        if new_x != x || new_y != y || new_z != z || new_w != w {
                            result.push(Coord4D(new_x, new_y, new_z, new_w));
                        }
                    }
                }
            }
        }

        result
    }
}

fn get_inactive_adjacents<A: Adjacents>(active_coords: &HashSet<A>) -> HashSet<A> {
    active_coords.iter()
        .flat_map(|active| active.get_adjacent())
        .filter(|adj| !active_coords.contains(adj))
        .collect()
}

fn step<A: Adjacents>(active_coords: HashSet<A>) -> HashSet<A> {
    let inactive_coords = get_inactive_adjacents(&active_coords);
    let mut next = HashSet::new();

    for &active in active_coords.iter() {
        let active_neighbors = active.get_adjacent().iter()
            .filter(|&x| active_coords.contains(x))
            .count();

        if active_neighbors == 2 || active_neighbors == 3 {
            next.insert(active);
        }
    }

    for &inactive in inactive_coords.iter() {
        let active_neighbors = inactive.get_adjacent().iter()
            .filter(|&x| active_coords.contains(x))
            .count();

        if active_neighbors == 3 {
            next.insert(inactive);
        }
    }

    next
}

fn simulate<A: Adjacents>(mut active_coords: HashSet<A>) -> usize {
    for _ in 0..6 {
        active_coords = step(active_coords);
    }

    active_coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::vec_of_strings;

    #[test]
    fn test_solve() {
        let lines = vec_of_strings![
            ".#.",
            "..#",
            "###",
        ];

        let (part1, part2) = solve(&lines);
        assert_eq!(part1, 112);
        assert_eq!(part2, 848);
    }
}