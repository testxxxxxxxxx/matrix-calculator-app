use crate::matrix::Matrix;

pub struct MatrixOperations {

    matrix_a: Matrix,
    matrix_b: Matrix 

}

trait MatrixOperationsMethods {

    fn new(_matrix_a: Matrix, _matrix_c: Matrix) -> Self;
    fn add(&mut self) -> Matrix;
    fn sub(&mut self) -> Matrix;
    fn mul_a(&mut self, value: f32) -> Matrix;
    fn mul_a_and_b(&mut self) -> Matrix;
    fn rev(&mut self) -> Matrix;
    fn transform(&mut self) -> Matrix;
    fn det(&mut self) -> u32;

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
    fn sub(&mut self) -> Matrix {

        let mut matrix_c: Matrix = Matrix { rows: self.matrix_a.rows, col: self.matrix_a.col, matrix: Vec::new() };

        if self.matrix_a.rows != self.matrix_b.rows || self.matrix_a.col != self.matrix_b.col
        {

            return matrix_c;
        }

        for i in 0..matrix_c.rows
        {
            for j in 0..matrix_c.col
            {
                matrix_c.matrix[i as usize][j as usize] = self.matrix_a.matrix[i as usize][j as usize] - self.matrix_b.matrix[i as usize][j as usize];

            }

        }

        return matrix_c;
    }
    fn mul_a(&mut self, value: f32) -> Matrix {
        
        let mut matrix_c: Matrix = Matrix { rows: self.matrix_a.rows, col: self.matrix_a.col, matrix: Vec::new() };

        for i in 0..matrix_c.rows
        {
            for j in 0..matrix_c.col
            {
                matrix_c.matrix[i as usize][j as usize] *= value as usize;

            }

        }

        return matrix_c;

    }
    fn mul_a_and_b(&mut self) -> Matrix {

        let mut matrix_c: Matrix = Matrix { rows: self.matrix_a.rows, col: self.matrix_a.col, matrix: Vec::new() };
        let mut temp_result: f32;

        if self.matrix_a.rows != self.matrix_b.col || self.matrix_a.rows != self.matrix_b.col
        {

            return matrix_c;
        }

        for i in 0..matrix_c.rows
        {
            for j in 0..matrix_c.col
            {
                temp_result = 0.0;

                for x in 0..matrix_c.col
                {
                    temp_result += self.matrix_a.matrix[i as usize][x as usize] as f32 * self.matrix_b.matrix[x as usize][i as usize] as f32;

                }

                matrix_c.matrix[i as usize][j as usize] = temp_result as usize;

            }

        }

        return matrix_c;
    }
    fn rev(&mut self) -> Matrix {

        let mut matrix_c: Matrix = Matrix { rows: self.matrix_a.rows, col: self.matrix_a.col, matrix: Vec::new() };

        return matrix_c;
    }
    fn transform(&mut self) -> Matrix {

        let mut matrix_c: Matrix = Matrix { rows: self.matrix_a.col, col: self.matrix_a.rows, matrix: Vec::new() };

        for i in 0..matrix_c.rows
        {
            for j in 0..matrix_c.col
            {
                matrix_c.matrix[j as usize][i as usize] = self.matrix_a.matrix[i as usize][j as usize]; 

            }

        }

        return matrix_c;
    }
    fn det(&mut self) -> u32 {
        
        let mut result: u32;

        result = 0;


        return result;
    }
 

}