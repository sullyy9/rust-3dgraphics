//! Trait Providing an interface for Polygon types.
//!

use crate::{
    buffer::{Color, PixelBuffer, ZBuffer},
    geometry::LineSegment,
    rasterizer::{LineIter, Rasterize},
};

use super::geometry::{Dim, Point, Vector};

pub trait Polygonal {
    fn vertex_count(&self) -> usize;
    fn verticies(&self) -> &[&Point<f64, 4>];
    fn normal(&self) -> Vector<f64, 3>;
}

impl<T> Rasterize for T
where
    T: Polygonal,
{
    fn rasterize(&self, pixel_buffer: &mut PixelBuffer, z_buffer: &mut ZBuffer, color: &Color) {
        use Dim::{X, Y, Z};

        let mut verticies: Vec<Point<i32, 3>> = self
            .verticies()
            .iter()
            .map(|&&vertex| Point::from_homogenous(vertex))
            .map(|vertex| vertex.map(|coord| coord.round() as i32))
            .collect();

        let screen_width = (pixel_buffer.width - 1) as i32;
        let screen_height = (pixel_buffer.height - 1) as i32;

        // Order the verticies in increasing order of X.
        if verticies[1][X] <= verticies[0][X] && verticies[1][X] <= verticies[2][X] {
            verticies.swap(0, 1);
        } else if verticies[2][X] < verticies[0][X] && verticies[2][X] < verticies[1][X] {
            verticies.swap(0, 2);
        }
        if verticies[2][X] < verticies[1][X] {
            verticies.swap(1, 2);
        }

        let (ymin, ymax) = {
            let (mut min, mut max) = if verticies[0][Y] < verticies[1][Y] {
                (verticies[0][Y], verticies[1][Y])
            } else {
                (verticies[1][Y], verticies[0][Y])
            };

            if verticies[2][Y] < min {
                min = verticies[2][Y];
            } else if verticies[2][Y] > max {
                max = verticies[2][Y];
            }

            (
                min.clamp(0, screen_height) as usize,
                max.clamp(0, screen_height) as usize,
            )
        };

        /// X and Z must be usize as they will be used
        /// to index into the screen buffers.
        #[derive(Clone)]
        struct ScanlineBound {
            x: usize,
            z: usize,
        }
        let mut scanlines: Vec<Vec<ScanlineBound>> = vec![Vec::new(); (ymax - ymin) + 1];

        // Declare lines in clockwise order around the polygon but keep the leftmost point first.
        let mut lines = [
            LineSegment::new(verticies[0], verticies[1]),
            LineSegment::new(verticies[1], verticies[2]),
            LineSegment::new(verticies[0], verticies[2]),
        ];

        // Order the lines into the order they should be drawn.
        if (lines[0].gradient_xy().is_sign_positive() && lines[2].gradient_xy().is_sign_negative())
            || (lines[0].gradient_xy().is_sign_negative()
                && lines[2].gradient_xy().is_sign_positive())
        {
            lines.swap(1, 2);
        } else if lines[0].gradient_xy().abs() <= lines[2].gradient_xy().abs() {
            lines.swap(0, 2);
        }

        lines
            .into_iter()
            .flat_map(LineIter::from)
            .for_each(|point| {
                let x = point[X].clamp(0, screen_width) as usize;
                let y = point[Y].clamp(0, screen_height) as usize;
                let z = point[Z].clamp(0, i32::MAX) as usize;

                scanlines[y - ymin].push(ScanlineBound { x, z });
            });

        scanlines.into_iter().enumerate().for_each(|(y, scanline)| {
            let y = y + ymin;

            if let Some(start) = scanline.first() {
                let &ScanlineBound {
                    x: x1,
                    z: z1,
                } = start;
                let &ScanlineBound { x: x2, z: z2 } = scanline.last().unwrap();

                // It is known that dx will always be possitive so it's okay
                // to convert after the subtraction.
                // However dz may be negative.
                let zstep = {
                    let dx = (x2 - x1) as f64;
                    let dz = z2 as f64 - z1 as f64;
                    dz / dx
                };

                let mut z = z1 as f64;
                (x1..=x2).for_each(|x| {
                    if z < z_buffer[(x, y)] as f64 {
                        pixel_buffer[(x, y)].copy_from_slice(color.as_slice());
                        z_buffer[(x, y)] = z.round() as usize;
                    }
 
                    z += zstep;
                });
            }
        });
    }
}
