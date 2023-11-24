use std::fs;
pub fn run_02(message:Option<String>) {
    let contents;
    match message{
        Some(message) => {
            contents = message.clone().to_lowercase();
        },
        None => {
            contents = fs::read_to_string("src/challenge_02/input_02.txt")
                .expect("Failed to read").to_lowercase();
        }
    }
    let mut acc:i32 = 0;
    //println!("{}", contents);
    let op_list:Vec<&str> = contents.split("").filter(|&s| s != "").collect();
    for x in op_list{
        match x{
            "#" => acc += 1,
            "@" => acc -= 1,
            "*" => acc = acc.pow(2),
            "&" => print!("{:?}", acc),
            _ => eprintln!("Error parsing message.")
        }
    }
    println!();
}