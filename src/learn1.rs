struct User {
    active: bool,
    username: String,
    emial: String,
}


fn main() {

    // let ans = is_even(1000000000000);


    // let ans = fibonacii(5);

    // let name = String::from("narasimha");
    // let ans = get_string_leng_chars(&name);

    let ans = User {
        username: String::from("narasimha1511"),
        active: true,
        emial: String::from("this is naraismha")
    };

    println!("{}" , ans.emial);


}

// fn get_string_leng_chars(s: &str)  -> usize {
//     s.chars().count()
// }

// fn fibonacii(num: u64) -> u64{
    
//     if num == 0 {
//         return 0; 
//     }

//     if num == 1 {
//         return 1;
//     }


//     let mut first: u64  = 0;
//     let mut second: u64 = 1;

//     for _ in 0..(num - 1) {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }

//     return second;
// }


// fn is_even(num: u64) -> bool {
    
//     if num % 2 == 0 {
//         return true;
//     }

//     return false;
// }