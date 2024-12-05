pub fn solve(text_contents: Vec<String>) -> i32 {
    let (mut first_vec, mut second_vec) = split_into_two_vec(text_contents);

    first_vec.sort();
    second_vec.sort();

    let result = calculate_total_distance(first_vec, second_vec);
    result
}

fn split_into_two_vec(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut first_vec = Vec::new();
    let mut second_vec = Vec::new();

    for line in input {
        let mut parts = line.split_whitespace();
        if let Some(first) = parts.next() {
            if let Ok(parsed_first) = first.parse::<i32>() {
                first_vec.push(parsed_first);
            }
        }
        if let Some(second) = parts.next() {
            if let Ok(parsed_second) = second.parse::<i32>() {
                second_vec.push(parsed_second);
            }
        }
    }

    (first_vec, second_vec)
}

fn calculate_total_distance(first_vec: Vec<i32>, second_vec: Vec<i32>) -> i32 {
    let mut total_distance: i32 = 0;
    for (first, second) in first_vec.iter().zip(second_vec.iter()) {
        let difference = (first - second).abs();
        total_distance += difference;
    }
    total_distance
}
