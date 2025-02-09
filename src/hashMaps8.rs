use std::collections::HashMap;

/*
 * HashMaps
 * - insert
 * - get 
 * - remove 
 * - create
 *  
 */


 fn group_values_by_key(pairs: Vec<(&str , i32)>) -> HashMap<String , i32> {
    
    let mut hm = HashMap::new();

    for (key , value) in pairs {
        hm.insert(String::from(key), value);
    }

    return hm;
 }


fn _main(){
    
//    let mut users: HashMap<String , u32> = HashMap::new();

//     users.insert(String::from("nar"), 22);
//     users.insert(String::from("nar"), 32);
//     users.insert(String::from("nar"), 42);
//     users.insert(String::from("devil"), 36);
//     users.insert(String::from("devil"), 69);
//     users.insert(String::from("devil"), 66);
 

    // let get_nar = users.get("nara"); // this return an Option<22>

    // match get_nar {
    //     Some(data) => println!("this is the value for the key {}" , data),
    //     None => print!(" there is not such key in the database")
    // }


    let vec = vec![("narasimha" , 1),("narasimha" , 2),("narasimha" ,3),("af" , 5),("af" , 9)];

    let hm = group_values_by_key(vec);

    println!("{:?}" , hm );


}

fn main(){
println!("learing HashMaps ---------------- \n\n");
    _main();
print!("\n\ndone HaspMaps ----------------");
}

