use std::io;

fn main() {
    let mut first_calculation_number: i64 = 0;
    let mut second_calculation_number: i64 = 1;
    let mut result = 0;
    let mut trimmed_fibonacci_number: u64 = 0;
    let mut fibonacci_number = String::new();
    println!("Calculate the nth Fibonacci number");

    println!("Which Fibonacci number do you want to calculate?");

    loop {
        fibonacci_number.clear();
        if let Err(_) = io::stdin().read_line(&mut fibonacci_number) {
            println!("Couldn't read your input!");
        }
        match fibonacci_number.trim().parse::<u64>() {
            Ok(num) => {
                
                trimmed_fibonacci_number = num;
                break;
            }
            Err(_) => {
                println!(
                    "Please input a number which is able to be calculated to a Fibonacci number"
                );
            }
        }
    }
    if trimmed_fibonacci_number == 0 {
        println!("The 0th Fibonacci number is 0");
    } else if trimmed_fibonacci_number == 1 {
        println!("The 1st Fibonacci number is 1");
    } else {
        for _ in 1..trimmed_fibonacci_number {
            result = match first_calculation_number.checked_add(second_calculation_number) {
                Some(val) => val,
                None => {
                    println!("This Fibonacci number is to large to be calculatd by this programm");
                    return;
                }
            };
            first_calculation_number = second_calculation_number;
            second_calculation_number = result;
        }
        println!("The result is {result}");
    }
}
