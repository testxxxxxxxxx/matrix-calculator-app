use std::io;

pub struct Matrix{

    rows: u32,
    col: u32,
    matrix: Vec<Vec<usize>>

}

trait MatrixMethods {

    fn new(rows: u32, col: u32) -> Self;

    fn fill_matrix(&mut self);

}

impl MatrixMethods for Matrix {

    fn new(rows: u32, col: u32) -> Self {

        Matrix { rows: rows, col: col, matrix: Vec::new() }

    }
    fn fill_matrix(&mut self){

        let mut input_line = String::new();
        let mut value: usize = 0;

        for _i in 0..self.rows { 

           let mut vec: Vec<usize> = vec![0; self.rows.try_into().unwrap()];

           for _j in 0..self.col {

                let _ = io::stdin().read_line(&mut input_line);

                value = input_line.trim().parse().expect("Input not an integer");

                vec.push(value);

           }

           self.matrix.push(vec);

        }

    }

}