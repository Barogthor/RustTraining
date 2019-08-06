
// sort( [T], StrategyCmp)


pub fn select_sort<T>(array: &Vec<T>) -> Vec<T>
    where T: Clone + PartialOrd
{
    let mut sorted_array = array.to_vec();
    for i in 0..(sorted_array.len()-1){
        for j in (i+1)..sorted_array.len(){
            if sorted_array[j] < sorted_array[i]{
                let tmp = sorted_array[i].clone();
                sorted_array[i] = sorted_array[j].to_owned();
                sorted_array[j] = tmp;
            }
        }
    }
    sorted_array
}