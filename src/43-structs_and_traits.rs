mod geometry;
use geometry::{Point, Rectangle};

fn main() {
    let p = Point::new(0.0, 0.0);
    let q = Point::new(1.0, 1.0);
    let rectangle = Rectangle::new(p, q);
    
    let point = Point::new(1.0, 1.0);

    print_area(rectangle);
    print_area(point);
}

fn print_area<T : Shape>(shape : T) {
    println!("The area of this shape is: {}", shape.area());
}

// implementation of the trait
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
