use heapless::binary_heap::{BinaryHeap, Max,Min};
use esp_println::{self as _, print, println};
use defmt::info;

    let mut binary_que : BinaryHeap<&str,Max,8888> = BinaryHeap::new(); // Change min if you want to start with min and change to max if you want to start with max on top 
        // Here max meenas reverse alpahateic order Z-A
        // Min means alaphametic order so A-Z


    binary_que.push("B").unwrap();
    binary_que.push("C").unwrap();
    binary_que.push("A").unwrap();
    binary_que.push("F").unwrap();
    binary_que.push("D").unwrap();


    while !binary_que.is_empty() {
        let popped_value = binary_que.pop();
        println!("{:?}",popped_value);
          let result =   match popped_value {
                Some::<&str>(a) =>{
                    
                    match  a {
                        "A" =>  "Gold",
                        "B" => "Silver",
                        "C" => "Bronze",
                        _ => "NOTHING"
                    }
                }
                None =>{
                    "Nothing"
                }
               
            };
            
            println!(" Popped value =>{:?} Result =>  {:?}",popped_value.unwrap_or("FAiled To unwrap"),result);
            }

        


