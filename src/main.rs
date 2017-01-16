mod shape;
mod rectangle;
mod circle;
mod triangle;
mod shape_factory;

use shape_factory::ShapeFactory;

fn main() {
    let shape_factory = ShapeFactory::new();
    let shape1_option = shape_factory.get_shape("CIRCLE");
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
