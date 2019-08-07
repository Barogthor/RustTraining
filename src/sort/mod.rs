
// sort( [T], StrategyCmp)


pub fn select_sort<T>(array: &mut [T], cmp_closure: Option<Box<&Fn(&T,&T)->bool>>)
    where T: Clone + PartialOrd
{
    let cmp_ord = match cmp_closure {
        Option::Some(closure) => *closure,
        None => &|a:&T,b:&T|   *a<*b
    };
    let len = array.len();
    for i in 0..(len-1){
        let mut tmp = i;
        for j in (i+1)..len{
            if cmp_ord(&array[j], &array[tmp]){
                tmp=j;
            }
        }
        array.swap(i,tmp);
    }

}