use std::io;

pub struct Matrix{

    pub rows: u32,
    pub col: u32,
    pub matrix: Vec<Vec<isize>>

}

pub trait MatrixMethods {

    fn new(rows: u32, col: u32) -> Self;

    fn fill(&mut self);

    fn fill_unit(&mut self);

    fn print(&self);

}

impl MatrixMethods for Matrix {

    fn new(rows: u32, col: u32) -> Self {

        Matrix { rows: rows, col: col, matrix: Vec::new() }

    }
    fn fill(&mut self){

        let mut input_line = String::new();
        let mut value: isize;

        for _i in 0..self.rows { 

           let mut vec: Vec<isize> = vec![0; self.rows.try_into().unwrap()];

           for _j in 0..self.col {

                let _ = io::stdin().read_line(&mut input_line);

                print!("{ }", input_line);

                value = input_line.trim().parse().expect("Input not an integer");

                vec.push(value);

           }

           self.matrix.push(vec);

        }

    }

    fn fill_unit(&mut self){

        let mut it: u32 = 0;
        
        for _i in 0..self.rows {

            for _j in 0..self.col {

                if _i == it {
                    
                    self.matrix[_i as usize][_j as usize] = 1;

                }
                else {
                    
                    self.matrix[_i as usize][_j as usize] = 0;

                }

            }

            it += 1;

        }

    }

    fn print(&self) {

        for _i in 0..self.rows {

            print!("\n");

            for _j in 0..self.col {

                print!("%d{}", self.matrix[_i as usize][_j as usize]);

            }

        }

    }

}