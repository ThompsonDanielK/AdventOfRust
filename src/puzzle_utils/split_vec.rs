pub fn split_vec(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
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
