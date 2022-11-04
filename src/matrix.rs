pub fn create_matrix(size: usize) -> Vec<Vec<String>> {
    let matrix = vec![vec![" ".to_string(); size]; size];
    println!("{:?}", matrix);

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
    println!("{:?}", matrix);
    draw_board(matrix.to_vec());
    return true;
}
