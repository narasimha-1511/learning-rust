
/*
 * learn about Options/Results Enum 
 * 1. error handling 
 * 2. null values or pointers
 * 
 */



 fn main(){
    println!("learing enums ---------------- \n\n");
    
        let  naraismha = String::from("narasimha");
        let indd = find_first_a(&naraismha);
    
        match indd {
            Some(value) => println!("this is the result for narasimha {}" , value),
            None => println!(" a not found")
        }
    
        let preet = String::from("preet");
        let ind = find_first_a(&preet);
    
        match ind {
            Some(value) => println!("this is the result for preet {}" , value),
            None => println!(" a not found")
        }
    
    println!("\n\ndone enums ----------------");
    }
    
    
    /*
     * Options Enum
     * -> you can return None/Null/Some_datatype 
     * 
     */
    fn find_first_a(str :&str) -> Option<usize> {
    
        for (index , char) in str.chars().enumerate() {
            if char == 'a' {
                return Some(index);
            }
        }
    
        return None;
    }