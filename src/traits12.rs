
//Traits -> similar to interfaces


pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("hi there")
    }
}

struct User {
    name : String ,
    age : i32
}

struct Fix {}

impl Summary for User {
    // here you are over riding and then you get this otherwise
    // try removing this code and see what it exactly runs
    fn summarize(&self) -> String {
        return format!(" User {} is {} years old", self.name , self.age);
    }
}
impl Summary for Fix{}
impl Summary for String{}

fn _main(){

    let user = User {
        name : String::from("narasimha"),
        age: 19
    };

    let fix = Fix{};

    //lets pass both fix and user
    notify(&fix);
    notify(&user);
    notify(&String::from("hello there"));


}

//any thing that implement the  summary traight can be passed here
// the sytax of this internally gets converted to something else known as "Trait Bound Syntax"
/*
 * these inside them selves use the T generics
 * 
 */
fn notify(u: &impl Summary){
    println!("{}",u.summarize())
}


fn main(){
println!("Learing Traits ---------------- \n\n");
    _main();
print!("\n\ndone Traits ----------------");
}