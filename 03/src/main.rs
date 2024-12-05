use regex::Regex;

#[cfg(test)]
const INPUT: &str = include_str!("../example");

#[cfg(not(test))]
const INPUT: &str = include_str!("../input");

fn solution_1() -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(INPUT)
        .map(|c| {
            let (i, [op1, op2]) = c.extract();
            println!("{}", i);
            op1.parse::<i32>().unwrap() * op2.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}

fn solution_2() -> i32 {
    let re = Regex::new(r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))")
        .unwrap();
    let mut res = 0;
    let mut valid = true;
    for m in re.captures_iter(INPUT) {
        if m.name("d").is_some() {
            valid = true;
        } else if m.name("dt").is_some() {
            valid = false;
        }
        if valid {
            if let (Some(op1), Some(op2)) = (m.name("op1"), m.name("op2")) {
                res += op1.as_str().parse::<i32>().unwrap() * op2.as_str().parse::<i32>().unwrap();
            }
        }
    }
    res
}

fn main() {
    println!("solution 1: {}", solution_1());
    println!("solution 2: {}", solution_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let result = solution_1();
        assert_eq!(result, 161);
    }

    #[test]
    fn test_solution_2() {
        let result = solution_2();
        assert_eq!(result, 48);
    }
}
