use std::fs::read_to_string;

fn is_safe(report: &[i32]) -> bool {
    let is_increasing = report.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = report.windows(2).all(|w| w[0] > w[1]);
    let has_valid_difference = report.windows(2).all(|w| {
        let difference = (w[0] - w[1]).abs();
        (1..=3).contains(&difference)
    });

    (is_increasing || is_decreasing) && has_valid_difference
}

fn is_safe_dampender(report: &[i32]) -> bool {
    // Check if the whole report is safe
    if is_safe(report) {
        return true;
    }

    // Allow one bad level by removing it and checking the remaining report
    (0..report.len()).any(|i| {
        let mut filtered = report.to_vec();
        filtered.remove(i);
        is_safe(&filtered)
    })
}

fn main() {
    let filepath = "./input";
    // let filepath = "./example";
    let reports: Vec<Vec<i32>> = read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    println!(
        "Question 1: {:?}",
        reports.iter().filter(|x| is_safe(x)).count()
    );
    println!(
        "Question 2: {:?}",
        reports.iter().filter(|x| is_safe_dampender(x)).count()
    );
}
