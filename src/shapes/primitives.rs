#[derive(Copy, Clone)]
pub struct Point(pub f32, pub f32, pub f32, pub f32);
impl Point {
    /// Perform a transformation with the point using a transformation matrix and return the result.
    pub fn transform_to_copy(&self, array: &[[f32; 4]; 4]) -> Point {
        Point(
            self.0 * array[0][0]
                + self.1 * array[1][0]
                + self.2 * array[2][0]
                + self.3 * array[3][0],
            self.0 * array[0][1]
                + self.1 * array[1][1]
                + self.2 * array[2][1]
                + self.3 * array[3][1],
            self.0 * array[0][2]
                + self.1 * array[1][2]
                + self.2 * array[2][2]
                + self.3 * array[3][2],
            self.0 * array[0][3]
                + self.1 * array[1][3]
                + self.2 * array[2][3]
                + self.3 * array[3][3],
        )
    }

    /// Perform a transformation on the point using a transformation matrix.
    pub fn transform(&mut self, array: &[[f32; 4]; 4]) {
        let old_self = *self;
        *self = Point(
            old_self.0 * array[0][0]
                + old_self.1 * array[1][0]
                + old_self.2 * array[2][0]
                + old_self.3 * array[3][0],
            old_self.0 * array[0][1]
                + old_self.1 * array[1][1]
                + old_self.2 * array[2][1]
                + old_self.3 * array[3][1],
            old_self.0 * array[0][2]
                + old_self.1 * array[1][2]
                + old_self.2 * array[2][2]
                + old_self.3 * array[3][2],
            old_self.0 * array[0][3]
                + old_self.1 * array[1][3]
                + old_self.2 * array[2][3]
                + old_self.3 * array[3][3],
        )
    }
}

#[derive(Copy, Clone)]
pub struct Line(pub Point, pub Point);
impl Line {
    pub fn new(point1: &Point, point2: &Point) -> Line {
        Line(
            Point(point1.0, point1.1, point1.2, point1.3),
            Point(point2.0, point2.1, point2.2, point2.3),
        )
    }
}

pub struct Triangle(pub Point, pub Point, pub Point);
impl Triangle {
    pub fn new(point1: &Point, point2: &Point, point3: &Point) -> Triangle {
        Triangle(
            Point(point1.0, point1.1, point1.2, point1.3),
            Point(point2.0, point2.1, point2.2, point2.3),
            Point(point3.0, point3.1, point3.2, point3.3),
        )
    }

    pub fn get_lines(&self) -> [Line; 3] {
        [
            Line::new(&self.0, &self.1),
            Line::new(&self.1, &self.2),
            Line::new(&self.2, &self.0),
        ]
    }
}
