use std::process::{self};

pub fn create_matrix(size: usize) -> Vec<Vec<String>> {
    clear_board();
    let matrix = vec![vec![" ".to_string(); size]; size];

    return matrix;
}

fn draw_row(row: Vec<String>) {
    for _i in 0..row.len() {
        print!("- - - - ");
    }
    println!("");
    for _i in 0..row.len() {
        print!("|     |");
    }
    println!("");
    for i in 0..row.len() {
        print!("|  {}  |", row[i]);
    }
    println!("");
    for _i in 0..row.len() {
        print!("|     |");
    }
    println!("");
    for _i in 0..row.len() {
        print!("- - - - ");
    }
    println!("");
}

pub fn draw_board(matrix: Vec<Vec<String>>) {
    for i in 0..matrix.len() {
        let row = matrix[i].clone();
        draw_row(row);
    }
}

pub fn clear_board() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn insert_letter(matrix: &mut Vec<Vec<String>>, letter: String, coord: Vec<usize>) -> bool {
    matrix[coord[0]][coord[1]] = letter;
    draw_board(matrix.to_vec());
    check_if_has_winner(matrix);
    return true;
}

fn check_diagonal(matrix: &Vec<Vec<String>>) -> bool {
    let current_letter = &matrix[0][0];
    if check_if_empty(current_letter) {
        return false;
    }
    for i in 0..matrix.len() {
        if &matrix[i][i] != current_letter {
            return false;
        }
    }
    return true;
}

fn check_reverse_diagonal(matrix: &Vec<Vec<String>>) -> bool {
    let current_letter = &matrix[matrix.len() - 1][0];
    if check_if_empty(current_letter) {
        return false;
    }
    for i in (0..matrix.len()).rev() {
        if &matrix[i][matrix.len() - 1 - i] != current_letter {
            return false;
        }
    }

    return true;
}

fn check_row(matrix: &Vec<Vec<String>>) -> bool {
    let mut response = false;
    for i in 0..matrix.len() {
        let current_letter = &matrix[i][0];
        if check_if_empty(current_letter) {
            response = false;
        }
        for j in 0..matrix.len() {
            if &matrix[i][j] != current_letter {
                response = false;
                break;
            }
            if (j == matrix.len() - 1)
                && &matrix[i][j] == current_letter
                && !check_if_empty(current_letter)
            {
                response = true;
                break;
            }
        }
        if response {
            break;
        }
    }
    return response;
}

fn check_column(matrix: &Vec<Vec<String>>) -> bool {
    let mut response = false;
    for i in 0..matrix.len() {
        let current_letter = &matrix[0][i];
        for j in 0..matrix.len() {
            if check_if_empty(&matrix[j][i]) {
                break;
            }
            if &matrix[j][i] != current_letter {
                response = false;
                break;
            }
            if (j == matrix.len() - 1) && &matrix[j][i] == current_letter {
                response = true;
                break;
            }
        }
    }
    return response;
}

fn check_if_empty(letter: &String) -> bool {
    if letter == &" ".to_string() {
        return true;
    }
    return false;
}

fn check_if_has_winner(matrix: &Vec<Vec<String>>) {
    if check_diagonal(matrix)
        || check_reverse_diagonal(matrix)
        || check_column(matrix)
        || check_row(matrix)
    {
        println!("A player has won!");
        exit_game();
    }
}

fn exit_game() {
    process::exit(0x0100);
}
