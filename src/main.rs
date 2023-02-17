use matrix::insert_letter;

mod input;
mod matrix;
mod utils;

fn main() {
    let matrix = matrix::create_matrix(3);
    let mut new_matrix = matrix.clone();
    matrix::draw_board(matrix);

    loop {
        let letter = input::get_input_choice();
        let coord = input::get_input_coord();
        matrix::clear_board();
        insert_letter(&mut new_matrix, letter, coord);
    }
}
