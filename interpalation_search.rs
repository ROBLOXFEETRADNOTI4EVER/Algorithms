pub async  fn interpalationsearch(data:&Vec<i32,999>,value:i32) -> Option<i32>{
    let mut  high = data.len() -1;
    let mut low = 0;

    while value >= data[low] && value <= data[high]  && low <= high{

        let mut  probe = low + (high - low)
         * (value as usize - data[low]  as usize ) / (data[high]  as usize  - data[low]  as usize );


    info!("probe {}",probe);

    if data[probe] == value{
        return Some(probe as i32)
    }
    else if data[probe] < value{
        low = probe + 1;
    }
    else {
        high = probe - 1;
    }
    }
    None
}



// example usage

let mut vec: Vec<i32, 999> = Vec::new(); // let mut vec: Vec<i32, 9999> = Vec::new(); CAN'T GO OVER 9999  SO Vec<i32,99999> CAN'T WORK

let mut i: i32 = 0;
for _ in 1..=300{
    i += 1;
    // info!("{}",i);
    vec.push(i).ok();

}
info!("interplation search{}",interpalationsearch(&vec,243).await);
