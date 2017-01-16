pub trait Shape {
    fn draw(&self);
}

use circle::Circle;
use rectangle::Rectangle;
use triangle::Triangle;

pub fn get_shape_by_name(shape_type: &str) -> Option<Box<Shape>> {
    let shape_type_uppercase = shape_type.to_uppercase();
    match shape_type_uppercase.as_ref() {
        "CIRCLE" => Some(Box::new(Circle)),
        "RECTANGLE" => Some(Box::new(Rectangle)),
        "TRIANGLE" => Some(Box::new(Triangle)),
        _ => None,
    }
}
