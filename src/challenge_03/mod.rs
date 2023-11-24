use std::fs;
pub fn run_03(message:Option<&str>){
    let data: String;
    let mut invalid_pass:Vec<&str> = vec!();
    match message{
        Some(value) => data = value.to_string(),
        None => data = read_data()
    }
    for l in data.lines(){
        if !is_valid(l){
            invalid_pass.push(l.split_ascii_whitespace().last().unwrap());
        }
    }
    //in case there are enough elements...
    println!("{:?}", invalid_pass.get(41).unwrap_or(&""));
}
fn read_data() -> String { 
    let data = fs::read_to_string("./src/challenge_03/input_03.txt")
                    .expect("Error reading file");
    data
}
fn is_valid(line: &str) -> bool{
    let line_components:Vec<&str> = line.split_whitespace().collect();
    let min_max_value:Vec<u32> = line_components[0].split("-").into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let letter = &line_components[1][0..1].chars().last().unwrap();
    //println!("{letter}");
    let num_ocur: &u32 = &line_components.last().unwrap().chars().fold(0,|mut acc, c| {
        if &c == letter{
            acc = acc + 1;
        }
        acc
    });
    //println!("{}",num_ocur);
    if num_ocur >= &min_max_value.get(0).unwrap() && num_ocur <= &min_max_value.get(1).unwrap(){
        true
    }
    else{
        false
    }
}