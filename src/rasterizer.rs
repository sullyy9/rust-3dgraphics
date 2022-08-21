use std::ops::RangeInclusive;

use crate::{
    buffer::{Color, PixelBuffer, ZBuffer},
    geometry::{Dim, LineSegment, Point},
};

pub trait Rasterize {
    fn rasterize(&self, pixel_buffer: &mut PixelBuffer, z_buffer: &mut ZBuffer, color: &Color);
}

pub struct LineIter {
    axes: [Dim; 3], // Ordered with the driving axis first.
    driving_range: RangeInclusive<i32>,
    axis_gain: [i32; 3],
    axis_delta: [i32; 3],
    axis_step: [i32; 3],
    this_point: Point<i32, 3>,
    next_point: Point<i32, 3>,
}

impl From<LineSegment<i32, 3>> for LineIter {
    fn from(line: LineSegment<i32, 3>) -> Self {
        use Dim::{X, Y, Z};

        let mut p1 = line.0;
        let mut p2 = line.1;

        let dx = p1[X].abs_diff(p2[X]) as i32;
        let dy = p1[Y].abs_diff(p2[Y]) as i32;
        let dz = p1[Z].abs_diff(p2[Z]) as i32;

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
            p1[axes[0]].abs_diff(p2[axes[0]]) as i32,
            p1[axes[1]].abs_diff(p2[axes[1]]) as i32,
            p1[axes[2]].abs_diff(p2[axes[2]]) as i32,
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
    type Item = Point<i32, 3>;

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
