#![no_std]
#![no_main]

use core::usize;

use alloc::string::String;
use alloc::vec;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::image::Image;
use esp_hal::clock::CpuClock;
use heapless::Deque;
use esp_hal::timer::timg::TimerGroup;
use esp_println::{self as _, print};
use heapless::Vec;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

struct  QUEQ{
    quequ: Deque<&'static str, 160>,
}
impl  QUEQ{
    pub async fn find_value(&mut self, value : &str) ->bool{
        self.quequ.iter().any(|&item| item == value)
        }

    pub async  fn find_value_position(&mut self,value:&str) -> Option<(usize,&'static str)> {
        self.quequ.iter().enumerate().find(|(_, &item) | item == value).map(|(idx,&item) | (idx,item))
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let mut peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");


    // Queue = Fifo data structure. First-In First-Out
        // A collection designed for holding elements prior to proccesing 
        // Linear data structure

    // add = enque push_back().ok()
    // remove = dequeq , pop_front() returns Option<u8>
   
    let mut queue: Deque<&str, 160> = Deque::new();    
    // inserting elements into the quee

    queue.push_back("42").ok(); 
    queue.push_back("42").ok(); 
    queue.push_back("42").ok(); 
    queue.push_back("42").ok(); 

    print!("\n {:?}",queue);
    let value = queue.pop_front(); 
    print!("\n value is {:?}",value);
    print!("\n que {:?}",queue);

    queue.push_front("Bob").ok();
    queue.push_front("Bob").ok();
    queue.push_front("Bob").ok();
    queue.push_front("Bob").ok();
    queue.push_front("Waldo").ok();

    queue.push_front("Bob").ok();
    queue.push_front("Bob").ok();

    queue.push_front("BobFront").ok();


    print!("\n queue {:?}",queue);
     print!("\n WheresWaldo {:?}",find_value_position(queue.clone(), "Waldo").await);
     print!("\n queue {:?}",queue);

}


pub async fn find_value(queue : Deque<&'static str, 160>, value : &str) ->bool{
    queue.iter().any(|&item| item == value)
    }

pub async  fn find_value_position(queue : Deque<&'static str, 160>, value : &str)-> Option<(usize,&'static str)> {
    queue.iter().enumerate().find(|(_, &item) | item == value).map(|(idx,&item) | (idx,item))
}
