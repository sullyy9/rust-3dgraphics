use std::{mem::swap, ops::RangeInclusive};

use crate::{
    geometry::{Dim, Point, Vector},
    mesh::Polygonal,
};


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
    pub x: isize,
    pub z: isize,
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
    pub ymin: isize,
    pub ymax: isize,

    pub normal: Vector<f64, 3>,
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

impl<T> From<T> for EdgeTable
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
            (min.round() as isize, max.round() as isize)
        };
        let mut table = vec![EdgeList::new(); ((ymax - ymin) + 1) as usize];

        // Declare lines in clockwise order around the polygon but keep the leftmost point first.
        let mut line1 = {
            let gradient =
                (vert[1][Dim::Y] - vert[0][Dim::Y]) / (vert[1][Dim::X] - vert[0][Dim::X]);
            (vert[0], vert[1], gradient)
        };
        let mut line2 = {
            let gradient =
                (vert[2][Dim::Y] - vert[1][Dim::Y]) / (vert[2][Dim::X] - vert[1][Dim::X]);
            (vert[1], vert[2], gradient)
        };
        let mut line3 = {
            let gradient =
                (vert[2][Dim::Y] - vert[0][Dim::Y]) / (vert[2][Dim::X] - vert[0][Dim::X]);
            (vert[0], vert[2], gradient)
        };

        // Order the lines into the order they should be drawn
        if (line1.2 > 0.0 && line3.2 < 0.0) || (line1.2 < 0.0 && line3.2 > 0.0) {
            swap(&mut line2, &mut line3);
        } else if line1.2.abs() < line3.2.abs() {
            swap(&mut line1, &mut line3);
        }

        LineIter::new(line1.0, line1.1).for_each(|point| {
            table[(point[Dim::Y] - ymin) as usize].push(XZPair {
                x: point[Dim::X],
                z: point[Dim::Z],
            });
        });
        LineIter::new(line2.0, line2.1).for_each(|point| {
            table[(point[Dim::Y] - ymin) as usize].push(XZPair {
                x: point[Dim::X],
                z: point[Dim::Z],
            });
        });
        LineIter::new(line3.0, line3.1).for_each(|point| {
            table[(point[Dim::Y] - ymin) as usize].push(XZPair {
                x: point[Dim::X],
                z: point[Dim::Z],
            });
        });

        EdgeTable {
            table,
            ymin,
            ymax,
            normal: Vector::default(),
        }
    }
}

pub struct LineIter {
    axes: [Dim; 3], // Ordered with the driving axis first.
    driving_range: RangeInclusive<isize>,
    axis_gain: [isize; 3],
    axis_delta: [isize; 3],
    axis_step: [isize; 3],
    this_point: Point<isize, 3>,
    next_point: Point<isize, 3>,
}

impl LineIter {
    pub fn new(p1: &Point<f64, 4>, p2: &Point<f64, 4>) -> LineIter {
        use Dim::{X, Y, Z};

        let mut p1 = Point::new([
            p1[Dim::X].round() as isize,
            p1[Dim::Y].round() as isize,
            p1[Dim::Z].round() as isize,
        ]);
        let mut p2 = Point::new([
            p2[Dim::X].round() as isize,
            p2[Dim::Y].round() as isize,
            p2[Dim::Z].round() as isize,
        ]);

        let dx = p1[X].abs_diff(p2[X]);
        let dy = p1[Y].abs_diff(p2[Y]);
        let dz = p1[Z].abs_diff(p2[Z]);

        let axes = if dx >= dy && dx >= dz {
            [Dim::X, Dim::Y, Dim::Z]
        } else if dy >= dx && dy >= dz {
            [Dim::Y, Dim::X, Dim::Z]
        } else {
            [Dim::Z, Dim::X, Dim::Y]
        };

        // Order the points so that as we travel along the line, we're moving in a positive
        // direction on the driving axis.
        if p1[axes[0]] > p2[axes[0]] {
            std::mem::swap(&mut p1, &mut p2);
        }

        let driving_range = p1[axes[0]]..=p2[axes[0]];
        let axis_step = [
            1,
            if p1[axes[1]] <= p2[axes[1]] { 1 } else { -1 },
            if p1[axes[2]] <= p2[axes[2]] { 1 } else { -1 },
        ];

        let axis_delta = [
            p1[axes[0]].abs_diff(p2[axes[0]]) as isize,
            p1[axes[1]].abs_diff(p2[axes[1]]) as isize,
            p1[axes[2]].abs_diff(p2[axes[2]]) as isize,
        ];
        let axis_gain = [
            0,
            (axis_delta[1] * 2) - axis_delta[0],
            (axis_delta[2] * 2) - axis_delta[0],
        ];

        LineIter {
            axes,
            driving_range,
            axis_gain,
            axis_delta,
            axis_step,
            this_point: Point::default(),
            next_point: p1,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point<isize, 3>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.driving_range.next() {
            self.this_point = self.next_point;
            self.next_point[self.axes[0]] = value;

            self.axes.iter().enumerate().skip(1).for_each(|(i, &dim)| {
                if self.axis_gain[i] > 0 {
                    self.next_point[dim] += self.axis_step[i];
                    self.axis_gain[i] -= self.axis_delta[0] * 2;
                }
                self.axis_gain[i] += self.axis_delta[i] * 2;
            });

            Some(self.this_point)
        } else {
            None
        }
    }
}
