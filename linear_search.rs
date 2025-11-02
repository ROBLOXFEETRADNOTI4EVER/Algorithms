  let mut array_ = [9,1,8,2,4,4,3,9,83,3,4,588238823];
  
    let index = linearsearch(array_,588238823).await;
    // Linear search
    info!("{}",index);
    loop {
        Timer::after_millis(500).await;

     
    }

async  fn linearsearch(array: [i32;12], value:i32 ) -> Option<i32> {
    let mut index = 0;
    for a in array{
        if a == value {
         return   Some(index)
        }
        index += 1
    }
    None
}
