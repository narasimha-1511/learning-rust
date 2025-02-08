struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    
    // this is like this we use in the tyoescript
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        2 *( self.width + self.height)
    }

    // there is no self here if you observe
    /*
     * as static functions are only attached to the class
     * we cannot call these functions on the object similary 
     * do you understand ... this is a really good way for you tyo revisse in the future
     */
    fn debug() -> i32 {
        return 1
    }

}


fn main(){

    let rect1 = Rect {
        width: 2,
        height: 2,
    };

   println!("area is {} \n permieter is {}" , rect1.area() , rect1.perimeter());
   println!("running debug {}" , Rect::debug())
}