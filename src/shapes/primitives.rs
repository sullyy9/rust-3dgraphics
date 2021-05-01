// #[derive(Copy, Clone)]
// pub struct Point {
//     pub x: u32,
//     pub y: u32,
//     pub z: u32,
//     pub w: u32,
// }

pub type Point<T> = [T; 4];

#[derive(Copy, Clone)]
pub struct Line {
    pub p1: Point<i32>,
    pub p2: Point<i32>,
}
impl Line {
    pub fn new(p1: Point<i32>, p2: Point<i32>) -> Line {
        Line { p1, p2 }
    }
}

pub struct Triangle {
    pub points: [Point<i32>; 3],
}
impl Triangle {
    pub fn new(point1: Point<i32>, point2: Point<i32>, point3: Point<i32>) -> Triangle {
        Triangle {
            points: [point1, point2, point3],
        }
    }

    pub fn get_lines(&self) -> [Line; 3] {
        [
            Line::new(self.points[0], self.points[1]),
            Line::new(self.points[1], self.points[2]),
            Line::new(self.points[2], self.points[0]),
        ]
    }
}