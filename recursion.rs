pub async  fn work(steps:i32) {
    // for i in 0..5{
    //     info!("SYBAU ADAM ")
    // }
    if steps < 1{ // base case
        info!("1 more reminign or stpes are 1");

        return;
    }
    info!("You take a step");

    Box::pin(work(steps -1 )).await

}
pub async  fn factorial(number:i32) -> i32{

    if number < 1 {
    
        return 1;
    }
   
    number *  Box::pin(factorial(number - 1)).await
    
}

pub async  fn power(x:i32,y:i32) ->i32{
    if y < 1 { return 1;
    }
    x * Box::pin(power(x, y - 1)).await
}

    // recursion
    work(1000000000).await;

    factorial(5).await;
    let  numb: i8 = 5;
    info!("{}'s factorial is -> {} ",numb,factorial(numb as i32).await);

    info!("Power of 4 and 5 is {}",{power(2,8).await});
