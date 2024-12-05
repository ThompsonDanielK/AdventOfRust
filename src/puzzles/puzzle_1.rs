pub fn solve(text_contents: Vec<String>) -> i32 {
    let (mut first_list, mut second_list) = split_into_two_lists(text_contents);

    first_list.sort();
    second_list.sort();

    let result = calculate_total_distance(first_list, second_list);
    result
}

fn split_into_two_lists(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input {
        let mut parts = line.split_whitespace();
        if let Some(first) = parts.next() {
            if let Ok(parsed_first) = first.parse::<i32>() {
                first_list.push(parsed_first);
            }
        }
        if let Some(second) = parts.next() {
            if let Ok(parsed_second) = second.parse::<i32>() {
                second_list.push(parsed_second);
            }
        }
    }

    (first_list, second_list)
}

fn calculate_total_distance(first_list: Vec<i32>, second_list: Vec<i32>) -> i32 {
    let mut total_distance: i32 = 0;
    for (first, second) in first_list.iter().zip(second_list.iter()) {
        let difference = (first - second).abs();
        total_distance += difference;
    }
    total_distance
}
