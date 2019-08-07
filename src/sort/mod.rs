
// sort( [T], StrategyCmp)


pub fn select_sort<T>(array: &Vec<T>, cmp_closure: Option<Box<&Fn(&T,&T)->bool>>) -> Vec<T>
    where T: Clone + PartialOrd
{
//    let default: &Fn(&T,&T)->bool =
    let cmp_ord = match cmp_closure {
        Option::Some(closure) => *closure,
        None => &|a:&T,b:&T|   *a<*b
    };
    let mut sorted_array = array.to_vec();
    for i in 0..(sorted_array.len()-1){
        for j in (i+1)..sorted_array.len(){
            if cmp_ord(&sorted_array[j], &sorted_array[i]){
                let tmp = sorted_array[i].to_owned();
                sorted_array[i] = sorted_array[j].to_owned();
                sorted_array[j] = tmp;
            }
        }
    }
    sorted_array
}