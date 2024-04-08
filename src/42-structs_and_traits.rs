mod geometry;
use geometry::{Point, Rectangle};

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Point {
    fn area(&self) -> f64 {
        0.0
    }
}

impl Shape for Rectangle {
    // Calculate the area of a rectangle
    fn area(&self) -> f64 {
        return self.width() * self.height();
    }
}

fn main() {
    let p = Point::new(0.0, 0.0);
    let q = Point::new(1.0, 1.0);
    let rectangle = Rectangle::new(p, q);

    let point = Point::new(5.0, 5.0);

    // Calculate and print the area of the point and rectangle
    println!("Area of the point: {}", point.area());
    println!("Area of the rectangle: {}", rectangle.area());
}
