extern crate rust_training;

#[cfg(test)]
mod matrix_tests{
    use super::rust_training::matrix::Matrix;

    #[test]
    fn matrix_mul_test(){
        let line1 = vec![1,5,4,2,5];
        let line2 = vec![4,9,4,0,2];
        let d2_array = vec![line1,line2];
        let matrix = Matrix::new(d2_array);
        let line1 = vec![1,5];
        let line2 = vec![3,9];
        let line3 = vec![6,4];
        let line4 = vec![0,7];
        let line5 = vec![8,2];
        let d2_array = vec![line1,line2, line3, line4, line5];
        let matrix2 = Matrix::new(d2_array);
        // [[80,90],[71,121]]
        let mul_matrix = matrix * matrix2;
        let expected_result = vec![vec![80,90],vec![71,121]];
        assert_eq!(expected_result, *mul_matrix.get_matrix());
    }

    #[test]
    fn matrix_mul_test2(){
        let matrix = Matrix::new(vec![
            vec![1,2],
            vec![3,4],
            vec![5,6],
            vec![7,8]
        ]);
        let matrix2 = Matrix::new(vec![
            vec![1,2,3],
            vec![4,5,6]
        ]);
        // [[80,90],[71,121]]
        let mul_matrix = matrix * matrix2;
        let expected_result: Vec<Vec<u32>> = vec![
            vec![9,12,15],
            vec![19,26,33],
            vec![29,40,51],
            vec![39,54,69]
        ];
        assert_eq!(expected_result, *mul_matrix.get_matrix());
    }
}