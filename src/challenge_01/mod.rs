use std::fs;
pub fn challenge_01() {
    let contents = fs::read_to_string("src/challenge_01/input_01.txt")
        .expect("Failed to read").to_lowercase();
    println!("{}",contents);
}
