use std::fs::read_to_string;

fn generate_operations(q: usize, n: usize) -> Vec<Vec<u8>> {
    let count = q.pow(n as u32);
    let mut all_operations = Vec::new();

    for i in 0..count {
        let mut operations = Vec::with_capacity(n);
        let mut num = i;

        for _ in 0..n {
            operations.push((num % q) as u8);
            num /= q;
        }
        all_operations.push(operations);
    }
    all_operations
}

fn evaluate_equation(operands: Vec<i64>, solution: Vec<u8>) -> i64 {
    let result = operands
        .iter()
        .skip(1)
        .zip(solution.iter())
        .fold(operands[0], |acc, (op, s)| match s {
            0 => acc + op,
            1 => acc * op,
            _ => panic!("invalid operand"),
        });
    result
}

fn evaluate_equation_str(operands: Vec<&str>, solution: Vec<u8>) -> i64 {
    let result = operands.iter().skip(1).zip(solution.iter()).fold(
        operands[0].to_string(),
        |acc, (op, s)| match s {
            0 => (acc.parse::<i64>().unwrap() + op.parse::<i64>().unwrap()).to_string(),
            1 => (acc.parse::<i64>().unwrap() * op.parse::<i64>().unwrap()).to_string(),
            2 => acc + op,
            _ => panic!("invalid operand"),
        },
    );
    result.parse().unwrap()
}

fn solution_1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            println!("line: {}", line);
            let (result, operands) = line.split_once(": ").unwrap();
            let result = result.parse::<i64>().unwrap();
            let operands = operands
                .split_whitespace()
                .map(|op| op.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            println!("operands: {:?}", operands);

            // check if the equation is valid by trying out inserting a * or +
            let positons = operands.len() - 1;
            let solutions = generate_operations(2, positons);
            let mut ret = 0;
            // println!("{:?}", solutions);
            for solution in solutions.iter() {
                let eqn_res = evaluate_equation(operands.clone(), solution.clone());
                if eqn_res == result {
                    println!("valid solution: {:?}", solution);
                    ret += result;
                    break;
                }
            }
            ret
        })
        .sum::<i64>()
}

fn solution_2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            println!("line: {}", line);
            let (result, operands) = line.split_once(": ").unwrap();
            let result = result.parse::<i64>().unwrap();
            let operands = operands.split_whitespace().collect::<Vec<&str>>();

            println!("operands: {:?}", operands);

            // check if the equation is valid by trying out inserting a * or +
            let positons = operands.len() - 1;
            let solutions = generate_operations(3, positons);
            let mut ret = 0;
            // println!("{:?}", solutions);
            for solution in solutions.iter() {
                let eqn_res = evaluate_equation_str(operands.clone(), solution.clone());
                if eqn_res == result {
                    println!("valid solution: {:?}", solution);
                    ret += result;
                    break;
                }
            }
            ret
        })
        .sum::<i64>()
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
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_solution_2() {
        let input = read_to_string("./example").unwrap();
        let result = solution_2(&input);
        assert_eq!(result, 11387);
    }
}
