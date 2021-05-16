use crate::primitives as prim;
use std::mem::swap;

/// Error handling
type Result<T> = std::result::Result<T, Error>;
pub enum Error {
    NoEdge,
}

/// A sub-struct of EdgeList containg x and z coordinates
#[derive(Clone)]
pub struct XZPair {
    pub x: i32,
    pub z: i32,
}

/// A sub-struct of EdgeTable containg a list of XZPair's
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

    /// Return the first and last edges from the list as an array
    ///
    /// # Errors
    /// NoEdge: No edges in the list
    pub fn get_edges(&self) -> Result<[&XZPair; 2],> {
        match self.list.first() {
            Some(pair1) => {
                let pair2 = self.list.last().unwrap();
                Ok([pair1, pair2])
            },
            None => Err(Error::NoEdge),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, XZPair> {
        self.list.iter()
    }
}

/// Edge table for the rasterization process.
pub struct EdgeTable {
    table: Vec<EdgeList>,
    pub ymin: i32,
    pub ymax: i32,

    pub normal: prim::Vector,
}
// Constructor function and helpers
impl EdgeTable {
    /// Create a new EdgeTable from a screen space polygon.
    pub fn new(mut poly: prim::OwnPolygon) -> EdgeTable {
        // Get the minimum and maximum y coordinate of the polygon. Limit it to screen space.
        let (ymin, ymax) = EdgeTable::min_max(poly.p1.y, poly.p2.y, poly.p3.y);
        let (ymin, ymax) = (ymin as i32, ymax as i32);

        // Order the polygon's verticies so that the left-most is first
        if poly.p2.x < poly.p1.x && poly.p2.x < poly.p3.x {
            swap(&mut poly.p1, &mut poly.p2);
        } else if poly.p3.x < poly.p1.x && poly.p3.x < poly.p2.x {
            swap(&mut poly.p1, &mut poly.p3);
        }
        if poly.p3.x < poly.p2.x {
            swap(&mut poly.p2, &mut poly.p3);
        }

        // Add an element to the table for each y cordinate
        let mut table = vec![EdgeList::new(); ((ymax - ymin) + 1) as usize];

        // Declare lines in clockwise order around the polygon but keep the leftmost point first.
        let line1_gradient = (poly.p2.y - poly.p1.y) / (poly.p2.x - poly.p1.x);
        let line2_gradient = (poly.p3.y - poly.p2.y) / (poly.p3.x - poly.p2.x);
        let line3_gradient = (poly.p3.y - poly.p1.y) / (poly.p3.x - poly.p1.x);

        let mut line1 = (poly.p1, poly.p2, line1_gradient);
        let mut line2 = (poly.p2, poly.p3, line2_gradient);
        let mut line3 = (poly.p1, poly.p3, line3_gradient);

        // Order the lines into the order they should be drawn
        if (line1.2 > 0.0 && line3.2 < 0.0) || (line1.2 < 0.0 && line3.2 > 0.0) {
            // Draw lines in order: 1, 3, 2
            swap(&mut line2, &mut line3);
        } else if line1.2.abs() < line3.2.abs() {
            // Draw lines in order: 3, 2, 1
            swap(&mut line1, &mut line3);
        } else {
            // Draw lines in order: 1, 2, 3
        }

        EdgeTable::draw_line(line1.0, line1.1, &mut table, ymin as i32);
        EdgeTable::draw_line(line2.0, line2.1, &mut table, ymin as i32);
        EdgeTable::draw_line(line3.0, line3.1, &mut table, ymin as i32);

        let normal = poly.normal;
        EdgeTable {
            table,
            ymin,
            ymax,
            normal,
        }
    }

    /// Return the minimum and maximum values of 3 parameters as a tuple (min, max)
    fn min_max(val1: f32, val2: f32, val3: f32) -> (f32, f32) {
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

    /// Draw a line into an edge table using brezenham's algorithm
    fn draw_line(p1: prim::Vertex, p2: prim::Vertex, table: &mut Vec<EdgeList>, yoffset: i32) {
        let (x1, y1, z1) = (p1.x as i32, p1.y as i32, p1.z as i32);
        let (x2, y2, z2) = (p2.x as i32, p2.y as i32, p2.z as i32);

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
                    y = y + ys;
                    ygain = ygain - (2 * dx);
                }
                if zgain > 0 {
                    z = z + zs;
                    zgain = zgain - (2 * dx);
                }

                ygain = ygain + (2 * dy);
                zgain = zgain + (2 * dz);
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
                    x = x + xs;
                    xgain = xgain - (2 * dy);
                }
                if zgain > 0 {
                    z = z + zs;
                    zgain = zgain - (2 * dy);
                }

                xgain = xgain + (2 * dx);
                zgain = zgain + (2 * dz);
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
                    x = x + xs;
                    xgain = xgain - (2 * dz);
                }
                if ygain > 0 {
                    y = y + ys;
                    ygain = ygain - (2 * dz);
                }

                xgain = xgain + (2 * dx);
                ygain = ygain + (2 * dy);
            }
        }
    }
}
impl EdgeTable {
    pub fn iter(&self) -> std::slice::Iter<'_, EdgeList> {
        self.table.iter()
    }
}
