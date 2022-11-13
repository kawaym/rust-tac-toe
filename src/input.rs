use std::io::{self};

pub fn get_input_choice() -> String {
    let word = get_input_from_stdin();

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
    println!("Primeiro número deve ser a linha e o segundo a coluna alvo");
    let binding = get_input_from_stdin();

    if binding.len() > 3 {
        println!("input should be no more than 3 characters");
        return [10000, 10000].to_vec();
    }

    let coord: Vec<usize> = binding
        .split(" ")
        .into_iter()
        .map(|c| c.parse::<usize>().unwrap() - 1)
        .collect();

    return coord;
}

fn get_input_from_stdin() -> String {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("reading from stdin failed");
    let binding = buff.trim().to_string();

    return binding;
}
