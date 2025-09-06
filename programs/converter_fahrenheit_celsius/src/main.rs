use std::io;

fn main() {
    println!("Convert Fahrenheit and Celsius!");

    println!("Do you want to convert to Fahrenheit or Celsius?");

    let mut user_input = String::new();

    let conversion_type: String;

    loop {
        user_input.clear();
        get_user_input(&mut user_input);

        let trimmed_user_input = user_input.trim().to_lowercase();
        if trimmed_user_input == "fahrenheit" || trimmed_user_input == "celsius" {
            conversion_type = trimmed_user_input;
            break;
        } else {
            println!("Please input a viable option.");
        }
    }
    println!("Which number do you want to convert?");
    let mut number = String::new();
    get_user_input(&mut number);
    let number: f64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error, Please enter a valid number");
            return;
        }
    };

    if conversion_type == "celsius" {
        let result = (number - 32.0) / 1.8;
        println!("{number} Fahrenheit is equal to {result} Celsius");
    }
    if conversion_type == "fahrenheit" {
        let result = number * 1.8 + 32.0;
        println!("{number} Celsius is equal to {result} Fahrenheit");
    }
}

fn get_user_input(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Couldn't read your input.");
}
