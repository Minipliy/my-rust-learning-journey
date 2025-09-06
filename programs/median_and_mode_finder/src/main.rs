use std::io;
use std::collections::HashMap;

fn main() {
    println!("Median and mode finder\nPlease input a list of integers like this: 2, 5, 10, 372, -72, ..., n");
    let user_integer_list = read_user_input();
    if let Some(numbers) = convert_to_vec(user_integer_list) {
        median_of_vector(&numbers);
        mode_of_vector(&numbers);
    } else {
        println!("Please input a viable list of integers!");
    }
}

fn mode_of_vector(vector: &Vec<i32>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in vector {
        let count: &mut i32 = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    for value in map.values() {
        if *value > max_count {
            max_count = *value;
        }
    }
    let modes: Vec<i32> = map.iter()
        .filter(|(_, v)| **v == max_count)
        .map(|(k, _)| *k)  // Fixed: dereference the key
        .collect();
    println!("The mode(s) of the given list is/are {:?}.", modes);
}
fn median_of_vector(vector: &Vec<i32>) {
    let mut sorted: Vec<i32> = vector.clone();
    sorted.sort();
    if sorted.len() % 2 == 0 {
        let median: f32 = (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) as f32 / 2.0;
        println!("The median of the given list is {}", median);
    } else {
        let median = sorted[sorted.len() / 2] as f32;  // Fixed: don't add the same element twice
        println!("The median of the given list is {}", median);
    }
}
fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read your input.");
    input
}

fn convert_to_vec(user_integer_list: String) -> Option<Vec<i32>> {
    let parts = user_integer_list.split(",");
    let mut numbers = Vec::new();
    for part in parts {
        let trimmed = part.trim();
        let parsed = trimmed.parse::<i32>();
        match parsed {
            Ok(parsed) => {
                numbers.push(parsed);
            }
            Err(_) => return None,
        }
    }
    Some(numbers)
}
