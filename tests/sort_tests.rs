extern crate rust_training;

#[cfg(test)]
mod sort_tests{
    use super::rust_training::sort::*;

    #[test]
    fn select_sort_int(){
        let values= vec![0,5,7,1,6,2,3,9,8,4];
        let expected_values = vec![0,1,2,3,4,5,6,7,8,9];
        let result = select_sort(&values, None);

        assert_eq!(result, expected_values);
    }

    #[test]
    fn select_sort_float(){
        let values= vec![0.0f32,5.0f32,7.0f32,1.0f32,6.0f32,2.0f32,3.0f32,9.0f32,8.0f32,4.0f32];
        let expected_values = vec![0.0f32,1.0f32,2.0f32,3.0f32,4.0f32,5.0f32,6.0f32,7.0f32,8.0f32,9.0f32];
        let result = select_sort(&values, None);

        assert_eq!(result, expected_values);
    }

    #[test]
    fn select_sort_with_closure(){
        let values= vec![0,5,7,1,6,2,3,9,8,4];
        let expected_values = vec![0,1,2,3,4,5,6,7,8,9];
        let clos: Option<Box<&Fn(&i32,&i32) -> bool>> = Option::Some(Box::new(&|a,b| *a<*b));
        let result = select_sort(&values, clos);

        assert_eq!(result, expected_values);
    }

    #[test]
    fn select_sort_with_closure_fail(){
        let values= vec![0,5,7,1,6,2,3,9,8,4];
        let expected_values = vec![0,1,2,3,4,5,6,7,8,9];
        let clos: Option<Box<&Fn(&i32,&i32) -> bool>> = Option::Some(Box::new(&|a,b| *a>*b));
        let result = select_sort(&values, clos);

        assert_ne!(result, expected_values);
    }
}

