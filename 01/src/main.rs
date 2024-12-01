use std::collections::HashMap;
use std::fs::read_to_string;

fn get_num_occurances(list: Vec<i32>) -> HashMap<i32, i32> {
    let mut num_map = HashMap::new();
    for num in list.iter() {
        let count = num_map.entry(*num).or_insert(0);
        *count += 1;
    }
    num_map
}

fn main() {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in read_to_string("./input").unwrap().lines() {
        let num = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        assert_eq!(num.len(), 2);
        left_list.push(num[0]);
        right_list.push(num[1]);
    }
    // sort the lists
    left_list.sort();
    right_list.sort();
    let right_map = get_num_occurances(right_list);

    let simillarity_score = left_list
        .iter()
        .map(|num| num * right_map.get(num).unwrap_or(&0))
        .sum::<i32>();
    println!("{:?}", simillarity_score);
    // println!("{:?}", left_map.get(&3).unwrap());
    // println!("{:?}", left_map.get(&3).unwrap());

    // // calculate the distance between the two numbers (absi value of the difference between the two numbers)
    // let distances = left_list
    //     .iter()
    //     .zip(right_list.iter())
    //     .map(|(x, y)| (x - y).abs())
    //     .collect::<Vec<i32>>();
    // let sum: i32 = distances.iter().sum();
    // println!("{:?}", sum);

    // let left = num[0];
    // let right = num[1];
    // // calculate the distance between the two numbers (absi value of the difference between the two numbers)
    // let distance = (left - right).abs();
    // disactances.push(distance);
    // let distances = vec![];
    // let sum: i32 = distances.iter().sum();
}
