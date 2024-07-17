use matrix::{Matrix, MatrixMethods};
use std::io;

mod matrix;
mod matrix_operations;

fn main() {
    
    print!("Podaj a: ");
    let mut input_value = String::new();
    let mut value: u32;

    let _ = io::stdin().read_line(&mut input_value);

    print!("{ }", input_value);

    let mut matrix_a: Matrix = Matrix{ rows: 5, col: 5, matrix: Vec::new() };

    let mut matrix_b: Matrix = Matrix{ rows: 5, col: 5, matrix: Vec::new() };


    matrix_b.fill();

    matrix_b.print();

}
