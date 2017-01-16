trait Shape {
    fn draw(&self);
}

struct Rectangle;
struct Circle;
struct Triangle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Inside Rectangle::draw function");
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Inside Circle::draw function");
    }
}

impl Shape for Triangle {
    fn draw(&self) {
        println!("Inside Triangle::draw function");
    }
}

struct ShapeFactory;

impl ShapeFactory {
    pub fn new() -> ShapeFactory {
        ShapeFactory
    }
    pub fn get_shape(&self, shape_type: &str) -> Option<Box<Shape>> {
        let shape_type_uppercase = shape_type.to_uppercase();
        match shape_type_uppercase.as_ref() {
            "CIRCLE" => Some(Box::new(Circle)),
            "RECTANGLE" => Some(Box::new(Rectangle)),
            "TRIANGLE" => Some(Box::new(Triangle)),
            _ => None,
        }
    }
}

fn main() {
    let shape_factory : ShapeFactory = ShapeFactory::new();
    let shape1_option : Option<Box<Shape>> = shape_factory.get_shape("CIRCLE");
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
