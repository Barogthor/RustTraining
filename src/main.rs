use rust_training::sort::select_sort;
use rand::Rng;

fn main() {
    let mut rand = rand::thread_rng();
//    let values: Vec<i64> = (0..100).map(|_| rand.gen_range(0,1000) ).collect();
    let values: Vec<i64> = vec![0,5,7,1,6,2,3,9,8,4];
    println!("{:?}", values);
    let sorted_array = select_sort(&values);
    println!("{:?}", sorted_array);

}
