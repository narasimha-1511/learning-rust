
/*
 * RUN -> cargo add chrono
 * to add these to that package manger 
 * 
 */

 use chrono::{Local , Utc};


// as i have thought , from where you are running the cargo 
// that is the root place of it to read hahah
fn _main(){
    
    //get the current time and date in UTC
    let now = Utc::now();
    println!("Current data and time in UTC: {}", now);
    

    //Format date and time 
    let formatted =  now.format("%y-%m-%d %H:%M:%S");
    println!("Formatted date and time : {} ", formatted);

    //get local time 
    let local = Local::now();
    println!("Current Date and time in local : {}" , local);
}



fn main(){
println!("learing result  enums ---------------- \n\n");
    _main();
println!("\n\ndone result enums ----------------");
}

