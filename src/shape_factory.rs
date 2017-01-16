use rectangle::Rectangle;
use circle::Circle;
use triangle::Triangle;
use shape::Shape;

pub struct ShapeFactory;

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
