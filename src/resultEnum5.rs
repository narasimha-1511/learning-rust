
/*
 * learn about Results Enum 
 * 1. error handling 
 * 
 */

 use std::fs::read_to_string;


 // as i have thought , from where you are running the cargo 
 // that is the root place of it to read hahah
 fn _main(){
     
     // let greeting_file_result = read_to_string("cargo.toml");
 
     // match greeting_file_result {
     //     Ok(data) => println!("{}",data),
     //     Err(err) => println!("{} , this is the error while reading the file" , err)
     // }
 
     // the best way to implemet a function 
     let ans = read_from_file_path(String::from("cargo.toml"));
 
     match ans {
         Ok(file_content) => {
             println!("File read SuccessFully : {:?}" , file_content);
         },
         Err(err) => {
             println!("Falied to read File: {:?}", err);
         }
     }
 }
 
 fn read_from_file_path(file_path : String) -> Result<String,String> {
 
     let result = read_to_string(file_path);
 
     match result {
         Ok(data) => Ok(data),
         Err(_err) => Err(String::from("File not read"))
     }
 }
 
 
 fn main(){
 println!("learing result  enums ---------------- \n\n");
     _main();
 println!("\n\ndone result enums ----------------");
 }
 
 