

// as i have thought , from where you are running the cargo 
// that is the root place of it to read hahah
fn _main(){
    
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(4);
    vec.push(1);

    ///easier way to do this 
    // let mut vec = Vec![1,2,3];

    even_filter(&mut vec);

    println!("{:?}" , vec);

}

fn even_filter(v : &mut Vec<i32>) {
    let mut i = 0;

    while i < v.len() {
        if v[i] % 2  != 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }    
}

// fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    
//     let mut new_vec = Vec::new();
    
//     for val in vec {
//         if val % 2 == 0 {
//             // here you are getting the address of that particular element 
//             // we need to use the startt to derefference
//             new_vec.push(*val);
//         }
//     }

//     return new_vec;
// }



fn main(){
println!("learing Vectors ---------------- \n\n");
    _main();
println!("\n\ndone Vectors ----------------");
}

