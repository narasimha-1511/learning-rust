
//  String and String Slices



/*
 * on Stack -> { name , ptr , len , capacity}
 * Strings
 * - create
 * - mutation
 * - deletion 
 */

 fn _main(){

    //Strings
  
    //create
    let mut namee = String::from("narasimha");
    namee.push_str(" god");
    println!("{}" , namee);

    //mutation
    namee.replace_range(9..namee.len(), " devil");
    println!("{}" , namee);
    
    //delete
    namee.replace_range(9..namee.len(), "");
    println!("{}" , namee);


    let name = String::from("naraismha is a good boy"); 

    //Learing String Slices
    let ans = get_first_word(&name);
    println!("\n\n {} ", ans);

    //This is called a string literal
    // you can seee the datatype here is `&str`
    let literal = "heloo"; 
    // if there is literal liek this is directly present in teh binary that is created 
    // this is literally hardcooded in the binary

}

//function take a string and returns a first word
fn get_first_word(str: &String) -> &str {
    let mut index =0;

    for (_ , i) in str.chars().enumerate() {
        if i == ' '{
            break;
        } 
        index += 1;
    }

    return &str[0..index];
}

fn main(){
println!("Learing String and Slices ---------------- \n\n");
    _main();
print!("\n\ndone String  and Slices ----------------");
}