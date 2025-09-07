use std::io;
use std::collections::HashMap;
fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
loop {
    let mut user_input = read_input();

    if user_input.to_lowercase().starts_with("add ") {
        let name_and_department = get_name_and_department_of_Add(&user_input);
        let department = company.entry(name_and_department.1).or_insert(Vec::new());
        department.push(name_and_department.0);
    } else if user_input.to_lowercase() == "list all" {
        for (key, value) in  &company {
            println!("{}: {}", key, value.join(", "));
        }
    } else if user_input.to_lowercase().starts_with("list ") {
        let department_name = get_name_of_department(&user_input);
        if let Some(employees) = company.get(&department_name) {
            println!("{}: {}" , department_name , employees.join(", "));
        } else {
            println!("There is no department named {}" , department_name);
        }
    } else if user_input.eq_ignore_ascii_case("exit")
           || user_input.eq_ignore_ascii_case("quit")
           || user_input.eq_ignore_ascii_case("stop") {
        break;
    } else {
        println!("Please input a valid command");
    }
}
}
fn read_input () -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read your Message");
    input.trim().to_string()
}
fn get_name_and_department_of_Add (input: &String) -> (String, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    (parts[1].to_string(), parts[3].to_string())
}
fn get_name_of_department(input: &String) -> String {
    let parts: Vec<&str> = input.split_whitespace().collect();
    parts[1].to_string()
}
