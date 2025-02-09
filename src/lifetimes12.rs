
//Lifetimes

// the code will not run to make yourself understand the lifetimes
// its very simple

fn _main(){
    
    let ans; 
    let a = String::from("small");
    
    // just remove this scope and see why there is no error 
    // and if the scope is there why there is a error
    {
        let b = String::from("longer");
        ans = longest(&a, &b);
    }
    
    println!("{}" , ans);
}

// Generic LifeTime annotation
// after writing this code , we can see the error in the main function which is actuvally crazy
fn longest<'a>(first: &'a str , second: &'a str) -> &'a str {
   if first.len() > second.len() {
    return first;
   } 
   return second
}

/*
 * at first the function accepted teh strigns as 'String' type directly 
 * see the old code below this
 * now we have changed them to &str 
 * and see why is it giving the error
 * 
 * 
 * here as i have did dry run it proves that
 * the ans in the main function is a dangling or a null pointer
 * 
 * so that is why we need to make sure tell the function
 * that what will be the life span of the inputs and what should be the lifespan of the outputs
 */

 // turned the old code to &str got an life time error

// fn longest (a: &str , b:&str) -> &str {
//     if a.len() > b.len() {
//         return a;
//     }
//     return b;
// }


// the old code

// fn longest (a: String , b:String) -> String {
//     if a.len() > b.len() {
//         return a;
//     }
//     return b;
// }

fn main(){
println!("Learing Lifetimes ---------------- \n\n");
    _main();
print!("\n\ndone Lifetimes  ----------------");
}