extern crate rust_training;

#[cfg(test)]
mod sort_tests{
    use super::rust_training::sort::*;

    #[test]
    fn select_sort_int(){
        let mut values= vec![0,5,7,1,6,2,3,9,8,4];
        let expected_values = vec![0,1,2,3,4,5,6,7,8,9];
        select_sort(&mut values, None);

        assert_eq!(values, expected_values);
    }

    #[test]
    fn select_sort_float(){
        let mut values= vec![0.0f32,5.0f32,7.0f32,1.0f32,6.0f32,2.0f32,3.0f32,9.0f32,8.0f32,4.0f32];
        let expected_values = vec![0.0f32,1.0f32,2.0f32,3.0f32,4.0f32,5.0f32,6.0f32,7.0f32,8.0f32,9.0f32];
        select_sort(&mut values, None);

        assert_eq!(values, expected_values);
    }

    #[test]
    fn select_sort_with_closure(){
        let mut values= vec![0,5,7,1,6,2,3,9,8,4];
        let expected_values = vec![0,1,2,3,4,5,6,7,8,9];
        let clos: Option<Box<&Fn(&i32,&i32) -> bool>> = Option::Some(Box::new(&|a,b| *a<*b));
        select_sort(&mut values, clos);

        assert_eq!(values, expected_values);
    }

    #[test]
    fn select_sort_with_closure_fail(){
        let mut values= vec![0,5,7,1,6,2,3,9,8,4];
        let expected_values = vec![0,1,2,3,4,5,6,7,8,9];
        let clos: Option<Box<&Fn(&i32,&i32) -> bool>> = Option::Some(Box::new(&|a,b| *a>*b));
        select_sort(&mut values, clos);

        assert_ne!(values, expected_values);
    }
}

