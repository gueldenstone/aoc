use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Map = Vec<Vec<char>>;
type Position = (i32, i32);

// fn print_current_map(map: &Map, guard_position: &(Position, Direction)) {
//     let mut map = map.clone();
//     let ((x, y), direction) = guard_position;
//     map[*y as usize][*x as usize] = match direction {
//         Direction::Up => '^',
//         Direction::Down => 'v',
//         Direction::Left => '<',
//         Direction::Right => '>',
//     };
//     for row in map {
//         println!("{}", row.iter().collect::<String>());
//     }
//     println!();
// }

fn get_map(input: &str) -> (Map, (Position, Direction), HashSet<(i32, i32)>) {
    // parse the map for initial state
    // # is an obstacle
    // ^ is the starting point of the guard
    // . is an empty space
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard_position = ((0, 0), Direction::Up);
    let mut obstacles = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match *cell {
                '.' => {}
                '^' => {
                    guard_position = ((x as i32, y as i32), Direction::Up);
                }
                '>' => {
                    guard_position = ((x as i32, y as i32), Direction::Right);
                }
                '<' => {
                    guard_position = ((x as i32, y as i32), Direction::Left);
                }
                'v' => {
                    guard_position = ((x as i32, y as i32), Direction::Down);
                }
                '#' => {
                    obstacles.insert((x as i32, y as i32));
                }
                _ => panic!("invalid cell"),
            }
        }
    }
    (map, guard_position, obstacles)
}

fn solution_1(input: &str) -> i32 {
    let (map, guard_position, obstacles) = get_map(input);
    let (visited, _) = perform_walk(&map, &guard_position, &obstacles);
    (visited.len() - 1) as i32
}

fn perform_walk(
    map: &Map,
    initial_guard_position: &(Position, Direction),
    obstacles: &HashSet<Position>,
) -> (HashSet<Position>, bool) {
    let mut guard_position = initial_guard_position.clone();
    let map_width = map[0].len() as i32;
    let map_height = map.len() as i32;
    let mut visited: HashMap<Position, Vec<Direction>> = HashMap::new();
    while guard_position.0 .0 >= 0
        && guard_position.0 .0 < map_width
        && guard_position.0 .1 >= 0
        && guard_position.0 .1 < map_height
    {
        // print_current_map(map, &guard_position);

        let ((x, y), direction) = guard_position;
        let new_position = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        if obstacles.contains(&new_position) {
            // turn right
            let new_direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            guard_position = ((x, y), new_direction);
        } else {
            // move forward
            guard_position = (new_position, direction);
            let directions = visited.entry(guard_position.0).or_default();
            if directions.contains(&guard_position.1) {
                return (visited.keys().copied().collect::<HashSet<Position>>(), true);
            } else {
                directions.push(guard_position.1.clone());
            }
        }
    }
    (
        visited.keys().copied().collect::<HashSet<Position>>(),
        false,
    )
}

fn solution_2(input: &str) -> i32 {
    let (map, initial_guard_position, obstacles) = get_map(input);
    let (visited, _) = perform_walk(&map, &initial_guard_position, &obstacles);
    // visited.remove(&initial_guard_position.0);
    visited
        .into_par_iter()
        .filter(|&position| {
            println!("checking with obstacle at {:?}", position);
            let mut obstacles = obstacles.clone();
            obstacles.insert(position);
            let (_, looping) = perform_walk(&map, &initial_guard_position, &obstacles);
            looping
        })
        .count() as i32
}

fn main() {
    let input = read_to_string("./input").unwrap();

    println!("solution 1: {}", solution_1(&input));
    println!("solution 2: {}", solution_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = read_to_string("./example").unwrap();
        let result = solution_1(&input);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_solution_2() {
        let input = read_to_string("./example").unwrap();
        let result = solution_2(&input);
        assert_eq!(result, 6);
    }
}
