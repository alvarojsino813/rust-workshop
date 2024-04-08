mod geometry;
use geometry::{Point, Rectangle};

fn main() {
    let p = Point::new(0.0, 0.0);
    let q = Point::new(1.0, 1.0);
    let rectangle = Rectangle::new(p, q);

    println!("The width of the rectangle is {}", rectangle.width());
}
