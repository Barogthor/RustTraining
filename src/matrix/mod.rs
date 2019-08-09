use std::ops::Mul;
use core::ops;

#[derive(Debug)]
pub struct Matrix<T>{
    matrix: Vec<Vec<T>>,
    lines: usize,
    columns: usize
}

impl <T> Matrix<T>{

    pub fn count_lines(&self) -> usize{
        self.lines
    }
    pub fn count_columns(&self) -> usize{
        self.columns
    }
    pub fn get_matrix(&self) -> &Vec<Vec<T>> { &self.matrix}
}

impl Matrix<u32>{
    pub fn new(matrix: Vec<Vec<u32>>) -> Matrix<u32>{
        // Base 1d array
        let lines = matrix.len();
        let columns = matrix[0].len();
        for line in matrix.iter(){
            if line.len() != columns {
                panic!("matrix line size aren't uniform");
            }
        }
        Matrix{matrix, lines, columns}
    }

    fn sum_column(&self, column: usize) -> u32{
        let mut sum: u32 = 0;
        for line in self.matrix.iter(){
            sum+=line[column]
        }
        sum
    }

    fn sum_line(&self, line: usize) -> u32{
        let mut sum: u32 = 0;
        for column in self.matrix[line].iter(){
            sum+=*column
        }
        sum
    }

}

impl ops::Mul<Matrix<u32>> for Matrix<u32>{
    type Output = Matrix<u32>;

    fn mul(self, rhs: Matrix<u32>) -> Self::Output {
        if self.columns != rhs.lines{
            panic!("Dimension incorret, can't proceed");
        }
        let mut result = vec![vec![0;rhs.columns];self.lines];
        for x in 0..self.lines {
            for y in 0..rhs.columns {
                let mut sum=0;
                for z in 0..self.columns{
                    sum += self.matrix[x][z] * rhs.matrix[z][y]
                }
                result[x][y] = sum
            }
        }

        Matrix{matrix:result, lines: self.count_lines(), columns: rhs.count_columns()}
    }
}

