use std::collections::HashSet;
use std::fs::read_to_string;
use std::vec::Vec;

fn find_word(grid: &Vec<Vec<char>>, word: &str) -> Vec<(usize, usize, i32, i32)> {
    let directions = vec![
        (-1, 0),  // up
        (1, 0),   // down
        (0, -1),  // left
        (0, 1),   // right
        (-1, -1), // top-left diagonal
        (-1, 1),  // top-right diagonal
        (1, -1),  // bottom-left diagonal
        (1, 1),   // bottom-right diagonal
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let word_length = word_chars.len();
    let mut results = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;

                for i in 0..word_length {
                    let nr = r as i32 + i as i32 * dr;
                    let nc = c as i32 + i as i32 * dc;

                    // Check if out of bounds or character doesn't match
                    if nr < 0
                        || nr >= rows as i32
                        || nc < 0
                        || nc >= cols as i32
                        || grid[nr as usize][nc as usize] != word_chars[i]
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    results.push((r, c, dr, dc));
                }
            }
        }
    }

    results
}

fn solution_1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    find_word(&grid, "XMAS").len() as i32
}

fn solution_2(input: &str) -> i32 {
    // find a locations of A
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut instaces_count = 0;
    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, column)| {
            if *column == 'A' {
                // check the that we are not at the edge
                if r == 0 || r == grid.len() - 1 || c == 0 || c == row.len() - 1 {
                    return;
                }
                // get characters around the A character
                // top left
                let upper_row = grid.get(r - 1).unwrap();
                let lower_row = grid.get(r + 1).unwrap();
                let upper_left = upper_row.get(c - 1).unwrap();
                let lower_left = lower_row.get(c - 1).unwrap();
                let upper_right = upper_row.get(c + 1).unwrap();
                let lower_right = lower_row.get(c + 1).unwrap();
                // collect all the characters around the A character
                // upper_left and lower_right need to be M and S
                // upper_right and lower_left need to be M and S
                let left_right: HashSet<char> = HashSet::from_iter(vec![*upper_left, *lower_right]);
                let right_left: HashSet<char> = HashSet::from_iter(vec![*upper_right, *lower_left]);
                if left_right.iter().all(|c| *c == 'M' || *c == 'S')
                    && left_right.len() == 2
                    && right_left.iter().all(|c| *c == 'M' || *c == 'S')
                    && right_left.len() == 2
                {
                    println!("----------------");
                    println!("{}{}{}", upper_row[c - 1], upper_row[c], upper_row[c + 1]);
                    println!("{}{}{}", row[c - 1], row[c], row[c + 1]);
                    println!("{}{}{}", lower_row[c - 1], lower_row[c], lower_row[c + 1]);
                    println!("----------------");
                    instaces_count += 1;
                }
            }
        });
    });
    instaces_count
}

fn main() {
    let filepath = "./input";
    // let filepath = "./example";
    // separate rules from updates (separeted by empty line)

    let input = read_to_string(filepath).unwrap();
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
        assert_eq!(result, 18);
    }

    #[test]
    fn test_solution_2() {
        let input = read_to_string("./example").unwrap();
        let result = solution_2(&input);
        assert_eq!(result, 9);
    }
}
