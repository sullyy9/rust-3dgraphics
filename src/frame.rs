//use crate::shape;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::shapes::primitives;

pub type Colour = [u8; 4];
pub struct Frame {
    pixel_buffer: Pixels<Window>,

    // Screen parameters
    width: u32,
    height: u32,

    // Limits of the z axis in 3d space
    near_plane: u32,
    far_plane: u32,

    // Matrix to map 3d space to Normalized Device Coordinate (NDC) space.
    // This space is a cube where x and y coordinates are normalized between -1 and 1.
    // z is mapped between 0 and 1
    projection_matrix: [[f32; 4]; 4],
}
impl Frame {
    pub fn new(width: u32, height: u32, window: &Window) -> Frame {
        let pixel_buffer = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window);
            Pixels::new(width, height, surface_texture).expect("Failed to create pixel buffer")
        };

        let near_plane = 1;
        let far_plane = 1000;

        let x_mul = (2.0 * near_plane as f32) / width as f32;
        let y_mul = (2.0 * near_plane as f32) / height as f32;
        let z_mul = far_plane as f32 / (far_plane - near_plane) as f32;
        let z2_mul = z_mul * -1.0 * near_plane as f32;
        let projection_matrix = [
            [x_mul, 0.0, 0.0, 0.0],
            [0.0, y_mul, 0.0, 0.0],
            [0.0, 0.0, z_mul, 1.0],
            [0.0, 0.0, z2_mul, 0.0],
        ];

        Frame {
            pixel_buffer,
            width,
            height,
            near_plane,
            far_plane,
            projection_matrix,
        }
    }

    /// Clear the frame and draw a 2 pixel wide border.
    pub fn clear(&mut self) {
        let buffer = self.pixel_buffer.get_frame();
        for (i, pixel) in buffer.chunks_exact_mut(4).enumerate() {
            let x = (i as u32) % self.width;
            let y = (i as u32) / self.width;

            // Draw a white border 2 pixels wide
            if x <= 1 || x >= (self.width - 2) || y <= 1 || y >= (self.height - 2) {
                pixel.copy_from_slice(&[255, 255, 255, 255]);
            } else {
                pixel.copy_from_slice(&[0, 0, 0, 255]);
            }
        }
    }

    /// Project a 3D line to NDC space.
    pub fn draw_line_3d(&mut self, line_3d: &primitives::Line) {
        // Put the z value into the w element.
        let mut ndc_point1: primitives::Point<f32> = [
            (line_3d.p1[0] as f32 * self.projection_matrix[0][0]),
            (line_3d.p1[1] as f32 * self.projection_matrix[1][1]),
            (line_3d.p1[2] as f32 * self.projection_matrix[2][2])
                + (line_3d.p1[2] as f32 * self.projection_matrix[3][2]),
            (line_3d.p1[2] as f32 / 200.0),
        ];
        let mut ndc_point2: primitives::Point<f32> = [
            (line_3d.p2[0] as f32 * self.projection_matrix[0][0]),
            (line_3d.p2[1] as f32 * self.projection_matrix[1][1]),
            (line_3d.p2[2] as f32 * self.projection_matrix[2][2])
                + (line_3d.p2[2] as f32 * self.projection_matrix[3][2]),
            (line_3d.p2[2] as f32 / 200.0),
        ];

        ndc_point1[0] = ndc_point1[0] / ndc_point1[3];
        ndc_point1[1] = ndc_point1[1] / ndc_point1[3];
        ndc_point1[2] = ndc_point1[2] / ndc_point1[3];

        ndc_point2[0] = ndc_point2[0] / ndc_point2[3];
        ndc_point2[1] = ndc_point2[1] / ndc_point2[3];
        ndc_point2[2] = ndc_point2[2] / ndc_point2[3];

        let line_2d = {
            let point1: primitives::Point<i32> = [
                (ndc_point1[0] * self.width as f32) as i32,
                (ndc_point1[1] * self.height as f32) as i32,
                0,
                1,
            ];
            let point2: primitives::Point<i32> = [
                (ndc_point2[0] * self.width as f32) as i32,
                (ndc_point2[1] * self.height as f32) as i32,
                0,
                1,
            ];
            primitives::Line::new(point1, point2)
        };
        println!(
            "3d p1x: {}, p1y: {}, p1z: {}, p2x: {}, p2y: {}, p2z: {},",
            line_3d.p1[0],
            line_3d.p1[1],
            line_3d.p1[2],
            line_3d.p2[0],
            line_3d.p2[1],
            line_3d.p2[2]
        );
        println!(
            "2d p1x: {}, p1y: {}, p2x: {}, p2y: {}",
            line_2d.p1[0], line_2d.p1[1], line_2d.p2[0], line_2d.p2[1]
        );
        println!("");
        self.draw_line_2d(&line_2d);
    }

    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line_2d(&mut self, line: &primitives::Line) {
        let (x1, y1) = (line.p1[0], line.p1[1]);
        let (x2, y2) = (line.p2[0], line.p2[1]);

        let mut dx = (x2 - x1).abs();
        let mut dy = (y2 - y1).abs();

        // Increment through the axis with the greatest difference between points
        if dx > dy {
            // Increment through the X axis but make sure we're incrementing not decrementing.
            // Swap the points if necessary and recalculate dx and dy.
            if x1 > x2 {
                let (x1, y1) = (line.p2[0], line.p2[1]);
                let (x2, y2) = (line.p1[0], line.p1[1]);
                dx = (x2 - x1).abs();
                dy = (y2 - y1).abs();
            }

            // Decide if we need to increment or decrement y.
            let inc_dec = if y1 > y2 { -1 } else { 1 };

            let mut accumulated_y = (2 * dy) - dx;
            let mut y = y1;

            // Draw the line, incrementing/decrementing y as the accumulated change in y crosses the
            // threshold of what ammounts to a pixel.
            for x in x1..x2 {
                self.draw_pixel(x as u32, y as u32, [0, 255, 0, 255]);

                if accumulated_y > 0 {
                    y = y + inc_dec;
                    accumulated_y = accumulated_y - (2 * dx);
                }
                accumulated_y = accumulated_y + (2 * dy);
            }
        } else {
            // Increment through the Y axis but make sure we're incrementing not decrementing.
            // Swap the points if necessary and recalculate dx and dy.
            if y1 > y2 {
                let (x1, y1) = (line.p2[0], line.p2[1]);
                let (x2, y2) = (line.p1[0], line.p1[1]);
                dx = (x2 - x1).abs();
                dy = (y2 - y1).abs();
            }

            // Decide if we need to increment or decrement x.
            let inc_dec = if x1 > x2 { -1 } else { 1 };

            let mut accumulated_x = (2 * dx) - dy;
            let mut x = x1;

            // Draw the line, incrementing/decrementing x as the accumulated change in x crosses the
            // threshold of what ammounts to a pixel.
            for y in y1..y2 {
                self.draw_pixel(x as u32, y as u32, [0, 255, 0, 255]);

                if accumulated_x > 0 {
                    x = x + inc_dec;
                    accumulated_x = accumulated_x - (2 * dy);
                }
                accumulated_x = accumulated_x + (2 * dx);
            }
        }
    }

    /// Set a pixels colour via x and y coordinates. The origin is the bottom left corner.
    pub fn draw_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        let element = {
            let y_invert = self.height - y;
            (((y_invert * self.width) + x) * 4) as usize
        };

        let pixels = self
            .pixel_buffer
            .get_frame()
            .get_mut(element..(element + 4))
            .unwrap();

        pixels.copy_from_slice(&colour);
    }

    /// Render the pixel buffer to the screen.
    pub fn render(&mut self) {
        match self.pixel_buffer.render() {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to render pixel buffer")
            }
        }
    }

    /// Resize the surface the pixel buffer will be rendered on. This doesn't increase the
    /// resolution of the buffer. The individual pixels will get larger or smaller.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixel_buffer.resize(width, height);
        self.width = width;
        self.height = height;
    }
}
