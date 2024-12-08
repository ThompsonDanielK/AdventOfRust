use crate::puzzle_utils::split_vec::split_vec;

pub fn solve(text_contents: Vec<String>) -> i32 {
    let (mut first_vec, mut second_vec) = split_vec(text_contents);

    first_vec.sort();
    second_vec.sort();

    let result = calculate_total_distance(first_vec, second_vec);
    result
}

fn calculate_total_distance(first_vec: Vec<i32>, second_vec: Vec<i32>) -> i32 {
    let mut total_distance: i32 = 0;
    for (first, second) in first_vec.iter().zip(second_vec.iter()) {
        let difference = (first - second).abs();
        total_distance += difference;
    }
    total_distance
}
