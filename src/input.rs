use std::io::{self};

pub fn get_input_choice() -> String {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("reading from stdin failed");

    let word = buff.trim().to_string();

    if word.len() > 1 || (word.to_uppercase() != "X" && word.to_uppercase() != "O") {
        println!("input should be a X or O");
        println!("Aborting...");
        return "".to_string();
    }

    if word == ":quit" {
        return "".to_string();
    }

    return word;
}

pub fn get_input_coord() -> Vec<usize> {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("reading from stdin failed");
    let binding = buff.trim().to_string();

    if binding.len() > 3 {
        println!("input should be no more than 3 characters");
        return [10000, 10000].to_vec();
    }

    let coord: Vec<usize> = binding
        .split(" ")
        .into_iter()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    return coord;
}
