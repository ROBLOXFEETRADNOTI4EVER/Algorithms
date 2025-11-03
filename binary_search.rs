
let mut vec: Vec<i32, 999> = Vec::new(); // let mut vec: Vec<i32, 9999> = Vec::new(); CAN'T GO OVER 9999  SO Vec<i32,99999> CAN'T WORK

let mut i: i32 = 0;
for _ in 1..=300{
    i += 2;
    // info!("{}",i);
    vec.push(i).ok();

}

info!("{:?}",binarry_search_i32(&vec,30).await); // here a match statement would be good


}
pub async  fn binarry_search_i32(data_vec:&Vec<i32,999>,x: i32) ->Option<i32>{
    // halfing the array  and checking if its worked
    // also must return -1 if the array isn't sorted
        //later on add a helper function so it checks if its sorted and it starts
        // binary search is great in data types that are medium to big
let mut low :i32 = 0;

    let mut high = (data_vec.len() -1) as i32;
    while low <= high {
        let middle = low + (high - low) / 2;
        info!("middle is {}",middle);

        if data_vec[middle as usize] == x {
            return  Some(middle); // if the midle is the thing we want we returning the value 
        }
        else if data_vec[middle as usize] < x { // if the x is greater, ignore left half
            low = middle + 1;
        }
        else{ // if the x is smaller ignore right halg
            high = middle - 1
        }
        // if we reach here then the element wasn't present
      
    }
     None

    
}
