pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Line {
    pub point1: Point,
    pub point2: Point,
}
impl Line {
    pub fn new(point1: Point, point2: Point) -> Line {
        Line {
            point1,
            point2,
        }
    }
}