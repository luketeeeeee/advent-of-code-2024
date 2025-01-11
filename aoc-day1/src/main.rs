use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input").expect("ERROR");

    let lines: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("parse error"))
        .collect();

    let mut left_list: Vec<i32> = [].to_vec();
    let mut right_list: Vec<i32> = [].to_vec();

    for (index, line) in lines.iter().enumerate() {
        if index % 2 == 0 {
            left_list.push(*line);
        } else {
            right_list.push(*line);
        }
    }
    left_list.sort();
    right_list.sort();

    let mut results: Vec<i32> = [].to_vec();
    for (index, _value) in left_list.iter().enumerate() {
        let mut result = left_list[index] - right_list[index];

        if result < 0 {
            result = result * (-1);
        }

        results.push(result);
    }

    let total_distance: i32 = results.iter().sum();
    println!("{:?}", total_distance);
}
