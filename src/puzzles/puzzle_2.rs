use std::collections::HashMap;

use crate::puzzle_utils::split_vec::split_vec;

pub fn solve(text_contents: Vec<String>) -> i32 {
    let (first_vec, second_vec) = split_vec(text_contents);

    let mut frequency_map = HashMap::new();
    for &num in &first_vec {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = second_vec
        .iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
        .sum();

    similarity_score
}
