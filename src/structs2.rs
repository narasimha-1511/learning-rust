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

}


fn main(){

    let rect1 = Rect {
        width: 2,
        height: 2,
    };

   println!("area is {} \n permieter is {}" , rect1.area() , rect1.perimeter()) 
}