pub struct Point {
    pub x : f64,
    pub y : f64,
}

pub struct Rectangle {
    p: Point,
    q: Point,
}

impl Point {
    pub fn new(x : f64, y : f64) -> Point {
        return Point {
            x,
            y,
        }
    }

    pub fn add_x(&mut self, x : f64) {
        self.x += x;
    }
}

impl Rectangle {
    // Constructor method for Rectangle
    pub fn new(p: Point, q: Point) -> Self {
        Rectangle { 
            p,
            q
        }
    }

    pub fn width(&self) -> f64 {
        return if self.p.x > self.q.x {
            self.p.x - self.q.x
        } else {
            self.q.x - self.p.x
        }
    }

    pub fn height(&self) -> f64 {
        return if self.p.y > self.q.y {
            self.p.y - self.q.y
        } else {
            self.q.y - self.p.y
        }
    }
}
