use shape::Shape;

pub struct Rectangle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Inside Rectangle::draw function");
    }
}
