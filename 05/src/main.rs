use std::collections::{HashMap, HashSet};

use std::fs::read_to_string;

fn get_update(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

fn get_rule(line: &str) -> (i32, i32) {
    let parts: Vec<i32> = line
        .split('|')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    assert!(parts.len() == 2);
    (parts[0], parts[1])
}

fn separate_updates(
    updates: Vec<Vec<i32>>,
    rules: &HashMap<i32, Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let valid_updates = updates.iter().filter(|update| {
        let mut seen: HashSet<i32> = HashSet::new();
        for page in *update {
            if rules.contains_key(page) && !rules[page].iter().all(|rule| !seen.contains(rule)) {
                return false;
            }
            seen.insert(*page);
        }
        true
    });

    let invalid_updates = updates.iter().filter(|update| {
        let mut seen: HashSet<i32> = HashSet::new();
        for page in *update {
            if rules.contains_key(page) && !rules[page].iter().all(|rule| !seen.contains(rule)) {
                return true;
            }
            seen.insert(*page);
        }
        false
    });
    (
        valid_updates.map(|x| x.to_vec()).collect(),
        invalid_updates.map(|x| x.to_vec()).collect(),
    )
}

fn solution_1(input: &str) -> i32 {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = vec![];
    let mut is_update = false;
    for line in input.lines() {
        if line.is_empty() {
            is_update = true;
            continue;
        }
        if is_update {
            updates.push(get_update(line));
        } else {
            // parse the rule of the format 33|44
            let rule = get_rule(line);
            rules.entry(rule.0).or_default().push(rule.1);
        }
    }
    let (valid_updates, _invalid_updates) = separate_updates(updates, &rules);
    valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<i32>()
}

fn sort_updates(updates: &Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted_updates = vec![];
    for update in updates {
        let mut update_cpy = update.clone();
        update_cpy.sort_by(|a, b| {
            if rules.contains_key(a) && rules[a].contains(b) {
                std::cmp::Ordering::Less
            } else if rules.contains_key(b) && rules[b].contains(a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        sorted_updates.push(update_cpy);
    }
    sorted_updates
}

fn solution_2(input: &str) -> i32 {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = vec![];
    let mut is_update = false;
    for line in input.lines() {
        if line.is_empty() {
            is_update = true;
            continue;
        }
        if is_update {
            updates.push(get_update(line));
        } else {
            // parse the rule of the format 33|44
            let rule = get_rule(line);
            rules.entry(rule.0).or_default().push(rule.1);
        }
    }
    let (_valid_updates, invalid_updates) = separate_updates(updates, &rules);
    let invalid_updates = sort_updates(&invalid_updates, &rules);
    invalid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<i32>()
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
        assert_eq!(result, 143);
    }

    #[test]
    fn test_solution_2() {
        let input = read_to_string("./example").unwrap();
        let result = solution_2(&input);
        assert_eq!(result, 123);
    }
}
