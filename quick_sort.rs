pub async fn quick_sort(array:&mut Vec<i32,999>,start:isize,end:isize) {

   

    if end <= start{
        return ;
    }

    let mut pivot = partition(array, start, end);
    Box::pin(quick_sort(array,start,pivot - 1));
    Box::pin(quick_sort(array,pivot + 1,end)).await;

    // let mut j = array[0];
    // let mut i = array[0 - 1];

    // array.swap(j as usize, i as usize);




}
pub  fn partition(array:&mut Vec<i32,999>,start:isize,end:isize) -> isize{
    let  pivot: i32 = array[end as usize];
    let mut i: isize = start - 1 ;

    for mut j in  start..end  {
        if array[j as usize] <= pivot{
            i += 1;
            array.swap(i as usize, j as usize); 
        
        }
    }

    array.swap((i + 1) as usize, end as usize); 
    i + 1

}
