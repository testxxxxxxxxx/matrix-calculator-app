use crate::matrix::Matrix;

struct MatrixOperations {

    matrix_a: Matrix,
    matrix_b: Matrix 

}

trait MatrixOperationsMethods {

    fn new(_matrix_a: Matrix, _matrix_c: Matrix) -> Self;
    fn add(&mut self) -> Matrix;
    fn mul(&mut self) -> Matrix;
    fn rev(&mut self) -> Matrix;

}

impl MatrixOperationsMethods for MatrixOperations {

    fn new(_matrix_a: Matrix, _matrix_b: Matrix) -> Self {

        MatrixOperations { matrix_a: _matrix_a, matrix_b: _matrix_b }

    }
    fn add(&mut self) -> Matrix {

        let mut matrix_c: Matrix = Matrix { rows: self.matrix_a.rows, col: self.matrix_a.col, matrix: Vec::new() };

        if self.matrix_a.rows != self.matrix_b.rows || self.matrix_a.col != self.matrix_b.col
        {

            return matrix_c;
        }

        for i in 0..matrix_c.rows
        {
            for j in 0..matrix_c.col
            {
                matrix_c.matrix[i as usize][j as usize] = self.matrix_a.matrix[i as usize][j as usize] + self.matrix_b.matrix[i as usize][j as usize];

            }

        }

        return matrix_c;
    }
    fn mul(&mut self) -> Matrix {

        let matrix_c: Matrix = Matrix { rows: self.matrix_a.rows, col: self.matrix_a.col, matrix: Vec::new() };

        return matrix_c;
    }
    fn rev(&mut self) -> Matrix {

        let mut matrix_c: Matrix = Matrix { rows: self.matrix_a.rows, col: self.matrix_a.col, matrix: Vec::new() };

        return matrix_c;

    }


}