mod geometry;
use geometry::Point;

fn main() {

    let mut point = Point::new(1.0, 1.0);

    println!("({}, {})", point.x, point.y);
    
    point.add_x(4.0);

    println!("({}, {})", point.x, point.y);
}
