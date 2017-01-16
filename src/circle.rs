use shape::Shape;

pub struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Inside Circle::draw function");
    }
}
