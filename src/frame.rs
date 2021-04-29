use winit::window::Window;

use pixels::{Pixels, SurfaceTexture};

use crate::shape;

pub type Colour = [u8; 4];
pub struct Frame {
    pixel_buffer: Pixels<Window>,
    width: u32,
    height: u32,
}
impl Frame {
    pub fn new(width: u32, height: u32, window: &Window) -> Frame {
        let pixel_buffer = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window);
            Pixels::new(width, height, surface_texture).expect("Failed to create pixel buffer")
        };

        Frame {
            pixel_buffer,
            width,
            height,
        }
    }

    /// Draw a 2 pixel wide border in the frame.
    pub fn draw_border(&mut self) {
        let buffer = self.pixel_buffer.get_frame();
        for (i, pixel) in buffer.chunks_exact_mut(4).enumerate() {
            let x = (i as u32) % self.width;
            let y = (i as u32) / self.width;

            // Draw a white border 2 pixels wide
            if x <= 1 || x >= (self.width - 2) || y <= 1 || y >= (self.height - 2) {
                pixel.copy_from_slice(&[255, 255, 255, 255]);
            }
        }
    }

    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line(&mut self, line: &shape::Line) {
        let mut x1 = line.point1.x;
        let mut y1 = line.point1.y;
        let mut x2 = line.point2.x;
        let mut y2 = line.point2.y;

        let mut dx = ((x2 as i32) - (x1 as i32)).abs();
        let mut dy = ((y2 as i32) - (y1 as i32)).abs();

        // Increment through the axis with the greatest difference between points
        if dx > dy {
            // Increment through the X axis but make sure we're incrementing not decrementing.
            // Swap the points if necessary and recalculate dx and dy.
            if x1 > x2 {
                x1 = line.point2.x;
                y1 = line.point2.y;
                x2 = line.point1.x;
                y2 = line.point1.y;
                dx = ((x2 as i32) - (x1 as i32)).abs();
                dy = ((y2 as i32) - (y1 as i32)).abs();
            }

            // Decide if we need to increment or decrement y
            let inc_dec = if y1 > y2 { -1 } else { 1 };

            let mut accumulated_offset = (2 * dy) - dx;
            let mut y = y1;

            for x in x1..x2 {
                self.draw_pixel(x, y, [0, 255, 0, 255]);

                if accumulated_offset > 0 {
                    y = ((y as i32) + inc_dec) as u32;
                    accumulated_offset = accumulated_offset - (2 * dx);
                }
                accumulated_offset = accumulated_offset + (2 * dy);
            }
        } else {
            // Increment through the Y axis but make sure we're incrementing not decrementing.
            // Swap the points if necessary and recalculate dx and dy.
            if y1 > y2 {
                x1 = line.point2.x;
                y1 = line.point2.y;
                x2 = line.point1.x;
                y2 = line.point1.y;
                dx = ((x2 as i32) - (x1 as i32)).abs();
                dy = ((y2 as i32) - (y1 as i32)).abs();
            }

            // Decide if we need to increment or decrement x
            let inc_dec = if x1 > x2 { -1 } else { 1 };

            let mut accumulated_offset = (2 * dx) - dy;
            let mut x = x1;

            for y in y1..y2 {
                self.draw_pixel(x, y, [0, 255, 0, 255]);

                if accumulated_offset > 0 {
                    x = ((x as i32) + inc_dec) as u32;
                    accumulated_offset = accumulated_offset - (2 * dy);
                }
                accumulated_offset = accumulated_offset + (2 * dx);
            }
        }
    }

    /// Set a pixels value via x and y coordinate.
    pub fn draw_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        if x > self.width || x < 1 {
            println!("x: {}", x);
        }
        if y > self.height || y < 1 {
            println!("y: {}", y);
        }
        let element = (((y * self.width) + x) * 4) as usize;

        let pixels = self
            .pixel_buffer
            .get_frame()
            .get_mut(element..(element + 4))
            .unwrap();

        pixels.copy_from_slice(&colour);
    }

    /// Render the pixel buffer
    pub fn render(&mut self) {
        match self.pixel_buffer.render() {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to render pixel buffer")
            }
        }
    }

    /// Resize the surface the pixel buffer will be rendered on.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixel_buffer.resize(width, height);
        self.width = width;
        self.height = height;
    }
}
