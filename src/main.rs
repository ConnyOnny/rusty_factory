mod shape;
mod rectangle;
mod circle;
mod triangle;

use shape::Shape;

fn show_shape<T:Shape>(shape: &T) {
    println!("Have a look at this marvelous shape:");
    shape.draw();
    println!("Amazing!");
}

fn main() {
    let shape1_option = shape::get_shape_by_name("CIRCLE");
    match shape1_option {
        Some(shape1) => {
            // shape1 has type Box<Shape>
            shape1.draw();
        }
        None => {
            panic!("No shape found!");
        }
    }
    let shape2 = triangle::Triangle;
    show_shape(&shape2);
}
