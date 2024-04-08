#[derive(Clone, Copy)]
struct Point {
    x : f64,
    y : f64
}

fn main() {
    let point = Point{x : 0.0, y : 0.0};

    print_coordinates(point);
    print_coordinates(point);
}

fn print_coordinates(p : Point) {
    println!("({}, {})", p.x, p.y);
}
