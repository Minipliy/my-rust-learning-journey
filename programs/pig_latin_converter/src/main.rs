use std::io;

fn main() {

    println!("Pig Latin converter.\nEnter the text you want to have converted.\nFor Example: The bird is small");

    let mut input = user_input();

    string_to_pig_latin(&mut input);
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read your input!");
    input
}
fn string_to_pig_latin (input: &mut String) {
    let mut whole_text = Vec::new();
    let mut translated_text = Vec::new();
    let singular_text = input.trim().split(" ");
    for word in singular_text {
        let trimmed_word = word.trim();
        whole_text.push(word.to_string());
        }
    for words in whole_text {
        if matches!(words.chars().next() , Some('a' | 'e' | 'i' | 'o' | 'u')) {
            let translated_word = format!("{}-hay", words);
            translated_text.push(translated_word);
        } else {
           let Some(first_char) = words.chars().next() else {
               println!("Error!");
               return;
            };
            let shortend_word = words.chars().skip(1).collect::<String>();
            let translated_word = format!("{}-{}ay" , shortend_word, first_char);
            translated_text.push(translated_word);
       }
    }
    let result = translated_text.join(" ");
    println!("{}" , result);
}
