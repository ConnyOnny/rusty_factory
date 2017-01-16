use shape::Shape;

pub struct Triangle;

impl Shape for Triangle {
    fn draw(&self) {
        println!("Inside Triangle::draw function");
    }
}
