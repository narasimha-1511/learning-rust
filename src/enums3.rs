enum Shape  {
    Rectangle(f64 , f64),// width height
    Circle(f64),// radius
}

fn main(){
    println!("learing enums ----------------");

    let rect = Shape::Rectangle(2.0, 2.0);
    let rect_area = calculate_area(rect);

    let circle  = Shape::Circle(5.0);
    let circle_area  = calculate_area(circle);

    println!(" area of rectangle {} \n are of circle {} ", rect_area , circle_area);

    println!("done enums ----------------");
}

// this is called the patttern mathing in the enums
// like  aa switch case hahaha
fn calculate_area(shape : Shape) -> f64 {
    match shape  {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r *r,
    }
}