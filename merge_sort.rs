pub  async  fn merge_sort(array: &mut Vec<i32,999>){
    let n = array.len();

    if n <= 1 {  return ;}
    let mut middle = n / 2;
    let mut left_array: Vec<i32, 999> = Vec::new();
    let mut right_array: Vec<i32, 999> = Vec::new();



    for i in 0..middle {
        left_array.push(array[i]).unwrap();
    }
    
    for i in middle..n {
        right_array.push(array[i]).unwrap();
    }
    Box::pin(merge_sort(&mut left_array)).await;
    Box::pin(merge_sort(&mut right_array)).await;
    Box::pin(marge(left_array,right_array,array)).await

    

    }



  

    async fn marge(left_array:Vec<i32, 999>,right_array:Vec<i32, 999>,array: &mut Vec<i32, 999>){
        let mut left_size: usize = left_array.len();
        let mut right_size: usize = right_array.len();
        let mut i: usize = 0 ;
        let mut l: usize  = 0  ;
        let mut r: usize = 0 ;

        // check codinigton for merching
        while l < left_size && r < right_size {
            if left_array[l] < right_array[r] {
                array[i] = left_array[l];
                i += 1;
                l += 1;
            }
            else {
                array[i] = right_array[r];
                i += 1;
                r += 1;
            }

            
        }
        while l < left_size  {
            array[i] = left_array[l];
        i +=1;
        l += 1;
    }
    while  r < right_size {
        array[i] = right_array[r];
        i +=1;
        r +=1;
    }

    }
