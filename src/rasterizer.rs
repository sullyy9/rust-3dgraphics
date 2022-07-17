use crate::mesh::{
    geometry::{Dim, Vector},
    Point, Polygon, Polygonal,
};
use std::mem::swap;

///
/// Error handling
///
type Result<T> = std::result::Result<T, Error>;
pub enum Error {
    NoEdge,
}

///
/// A sub-struct of EdgeList containg x and z coordinates
///
#[derive(Clone)]
pub struct XZPair {
    pub x: i32,
    pub z: i32,
}

///
/// A sub-struct of EdgeTable containg a list of XZPair's
///
#[derive(Clone)]
pub struct EdgeList {
    list: Vec<XZPair>,
}
impl EdgeList {
    pub fn new() -> EdgeList {
        let list = Vec::new();
        EdgeList { list }
    }
}
impl EdgeList {
    pub fn push(&mut self, xzpair: XZPair) {
        self.list.push(xzpair);
    }

    ///
    /// Return the first and last edges from the list as an array
    ///
    /// # Errors
    /// NoEdge: No edges in the list
    ///
    pub fn get_edges(&self) -> Result<[&XZPair; 2]> {
        match self.list.first() {
            Some(pair1) => {
                let pair2 = self.list.last().unwrap();
                Ok([pair1, pair2])
            }
            None => Err(Error::NoEdge),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, XZPair> {
        self.list.iter()
    }
}

///
/// Edge table required for the rasterization process.
///
pub struct EdgeTable {
    table: Vec<EdgeList>,
    pub ymin: i32,
    pub ymax: i32,

    pub normal: Vector<3>,
}
// Constructor function and helpers
impl EdgeTable {
    ///
    /// Return the minimum and maximum values of 3 parameters as a tuple (min, max)
    ///
    fn min_max(val1: f64, val2: f64, val3: f64) -> (f64, f64) {
        let (mut min, mut max) = if val1 < val2 {
            (val1, val2)
        } else {
            (val2, val1)
        };

        if val3 < min {
            min = val3;
        } else if val3 > max {
            max = val3;
        }

        (min, max)
    }

    ///
    /// Draw a line into an edge table using brezenham's algorithm
    ///
    fn draw_line(p1: &Point<4>, p2: &Point<4>, table: &mut [EdgeList], yoffset: i32) {
        let (x1, y1, z1) = (p1[Dim::X] as i32, p1[Dim::Y] as i32, p1[Dim::Z] as i32);
        let (x2, y2, z2) = (p2[Dim::X] as i32, p2[Dim::Y] as i32, p2[Dim::Z] as i32);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let dz = (z2 - z1).abs();

        let xs = if x1 < x2 { 1 } else { -1 };
        let ys = if y1 < y2 { 1 } else { -1 };
        let zs = if z1 < z2 { 1 } else { -1 };

        if dx >= dy && dx >= dz {
            // X is the driving axis
            let mut ygain = (2 * dy) - dx;
            let mut zgain = (2 * dz) - dx;
            let mut y = y1;
            let mut z = z1;

            let xrange: Box<dyn Iterator<Item = _>> = if x1 < x2 {
                Box::new(x1..=x2)
            } else {
                Box::new((x2..=x1).rev())
            };

            for x in xrange {
                table[(y - yoffset) as usize].push(XZPair { x, z });

                if ygain > 0 {
                    y += ys;
                    ygain -= 2 * dx;
                }
                if zgain > 0 {
                    z += zs;
                    zgain -= 2 * dx;
                }

                ygain += 2 * dy;
                zgain += 2 * dz;
            }
        } else if dy >= dx && dy >= dz {
            // Y is the driving axis
            let mut xgain = (2 * dx) - dy;
            let mut zgain = (2 * dz) - dy;
            let mut x = x1;
            let mut z = z1;

            let yrange: Box<dyn Iterator<Item = _>> = if y1 < y2 {
                Box::new(y1..=y2)
            } else {
                Box::new((y2..=y1).rev())
            };

            for y in yrange {
                table[(y - yoffset) as usize].push(XZPair { x, z });

                if xgain > 0 {
                    x += xs;
                    xgain -= 2 * dy;
                }
                if zgain > 0 {
                    z += zs;
                    zgain -= 2 * dy;
                }

                xgain += 2 * dx;
                zgain += 2 * dz;
            }
        } else {
            // Z is the driving axis
            let mut xgain = (2 * dx) - dz;
            let mut ygain = (2 * dy) - dz;
            let mut x = x1;
            let mut y = y1;

            let zrange: Box<dyn Iterator<Item = _>> = if z1 < z2 {
                Box::new(z1..=z2)
            } else {
                Box::new((z2..=z1).rev())
            };

            for z in zrange {
                table[(y - yoffset) as usize].push(XZPair { x, z });

                if xgain > 0 {
                    x += xs;
                    xgain -= 2 * dz;
                }
                if ygain > 0 {
                    y += ys;
                    ygain -= 2 * dz;
                }

                xgain += 2 * dx;
                ygain += 2 * dy;
            }
        }
    }
}
impl EdgeTable {
    ///
    /// Iterate imutably over an EdgeList.
    ///
    pub fn iter(&self) -> std::slice::Iter<'_, EdgeList> {
        self.table.iter()
    }

    ///
    /// Iterate imutably over a slice of an EdgeList.
    ///
    pub fn iter_between(&self, first: usize, last: usize) -> std::slice::Iter<'_, EdgeList> {
        self.table[first..last].iter()
    }
}

impl<'a, T> From<T> for EdgeTable
where
    T: Polygonal,
{
    fn from(polygon: T) -> Self {
        let mut vert = polygon.verticies().to_vec();

        // Order the verticies in increasing order of X.
        if vert[1][Dim::X] < vert[0][Dim::X] && vert[1][Dim::X] < vert[2][Dim::X] {
            vert.swap(0, 1);
        } else if vert[2][Dim::X] < vert[0][Dim::X] && vert[2][Dim::X] < vert[1][Dim::X] {
            vert.swap(0, 2);
        }
        if vert[2][Dim::X] < vert[1][Dim::X] {
            vert.swap(1, 2);
        }

        // Add enough elements to the table to encompass the polygon in the Y axis
        let (ymin, ymax) = {
            let (min, max) = EdgeTable::min_max(vert[0][Dim::Y], vert[1][Dim::Y], vert[2][Dim::Y]);
            (min as i32, max as i32)
        };
        let mut table = vec![EdgeList::new(); ((ymax - ymin) + 1) as usize];

        // Declare lines in clockwise order around the polygon but keep the leftmost point first.
        let mut line1 = {
            let gradient = (vert[1][Dim::Y] - vert[0][Dim::Y]) / (vert[1][Dim::X] - vert[0][Dim::X]);
            (vert[0], vert[1], gradient)
        };
        let mut line2 = {
            let gradient = (vert[2][Dim::Y] - vert[1][Dim::Y]) / (vert[2][Dim::X] - vert[1][Dim::X]);
            (vert[1], vert[2], gradient)
        };
        let mut line3 = {
            let gradient = (vert[2][Dim::Y] - vert[0][Dim::Y]) / (vert[2][Dim::X] - vert[0][Dim::X]);
            (vert[0], vert[2], gradient)
        };

        // Order the lines into the order they should be drawn
        if (line1.2 > 0.0 && line3.2 < 0.0) || (line1.2 < 0.0 && line3.2 > 0.0) {
            swap(&mut line2, &mut line3);
        } else if line1.2.abs() < line3.2.abs() {
            swap(&mut line1, &mut line3);
        }

        EdgeTable::draw_line(line1.0, line1.1, &mut table, ymin as i32);
        EdgeTable::draw_line(line2.0, line2.1, &mut table, ymin as i32);
        EdgeTable::draw_line(line3.0, line3.1, &mut table, ymin as i32);

        EdgeTable {
            table,
            ymin,
            ymax,
            normal: Vector::default(),
        }
    }
}
