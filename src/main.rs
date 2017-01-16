mod shape;
mod rectangle;
mod circle;
mod triangle;

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
}
