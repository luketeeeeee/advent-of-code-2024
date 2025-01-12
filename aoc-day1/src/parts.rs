pub fn total_distance(left_list: Vec<i32>, right_list: Vec<i32>) {
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

pub fn similarity_score(left_list: Vec<i32>, right_list: Vec<i32>) {}
