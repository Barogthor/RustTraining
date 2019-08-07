use rust_training::sort::select_sort;
use rand::Rng;

fn test(a: i32, b: i32, f: Box<Fn(i32,i32) -> bool>) -> bool{
    (*f)(a,b)
}

fn main() {
    let mut rand = rand::thread_rng();
//    let values: Vec<i64> = (0..100).map(|_| rand.gen_range(0,1000) ).collect();
    let mut values: Vec<i64> = vec![0,5,7,1,6,2,3,9,8,4];

    println!("{:?}", values);
    select_sort(&mut values, None);
//    println!("{:?}", sorted_array);
    let clos: fn(i32,i32) -> bool = move |a,b| a<b;
    println!("test: {}",test(1,3,Box::new(clos)))

}
