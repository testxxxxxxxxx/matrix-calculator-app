pub struct Matrix{

    rows: u32,
    col: u32,
    matrix: Vec<usize>,

}

trait MatrixMethods {

    fn new(rows: u32, col: u32, matrix: Vec<usize>) -> Self;

    fn create(&mut self);

}

impl MatrixMethods for Matrix {

    fn new(rows: u32, col: u32, matrix: Vec<usize>) -> Self {

        Matrix { rows: rows, col: col, matrix: matrix}

    }
    fn create(&mut self){

        self.matrix = vec![0; (self.rows * self.col).try_into().unwrap()];

    }

}