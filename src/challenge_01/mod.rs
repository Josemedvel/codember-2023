use std::fs;
use std::collections::HashMap;
pub fn challenge_01(message:Option<String>) {
    let contents;
    match message{
        Some(message) => {
            contents = message.clone().to_lowercase();
        },
        None => {
            contents = fs::read_to_string("src/challenge_01/input_01.txt")
                .expect("Failed to read").to_lowercase();
        }
    }
    let word_array:Vec<&str> = contents.split_whitespace().collect();
    let mut word_order:Vec<&str> = Vec::new();
    let mut map:HashMap<&str, i32> = HashMap::new();
    for word in word_array{
        let old_value = map.get(word).cloned();
        match old_value {
            Some(value) => {
                map.insert(word, value + 1);
            },
            None => {
                map.insert(word, 1);
                word_order.push(word);
            }
        }
    }
    let mut result = String::new();
    for word in word_order{
        //result = result + word + map.get(word).expect("Error getting the value for the word").to_string().as_str(); //works
        result.push_str(&format!("{}{}", word, map.get(word).expect("Error getting the value for the word")));
    }
    println!("{}",result);
}
