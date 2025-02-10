
//Lifetimes With Structs

struct User<'a> {
    name : &'a str
  }
  
  fn _main(){ 
      let first_name = String::from("narasimha");
      let user  = User {
          name : &first_name
      };
  
      println!("the name of the user is {}" , user.name);
  }
  
  fn main(){
  println!("Learing Lifetimes ---------------- \n\n");
      _main();
  print!("\n\ndone Lifetimes  ----------------");
  }