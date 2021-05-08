use crate::shapes::primitives::{Line, Vertex};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

type Colour = [u8; 4];

pub struct GraphicsWindow {
    window: Window,
    size: PhysicalSize<u32>,

    pub pixel_buffer: Pixels,

    projection_matrix: [[f32; 4]; 4],
}
impl GraphicsWindow {
    pub fn new(width: u32, height: u32, event_loop: &EventLoop<()>) -> GraphicsWindow {
        let size = PhysicalSize::new(width, height);

        // Create the actual window
        let window = WindowBuilder::new()
            .with_title("3D Graphics")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();

        // Create a pixel buffer within the window
        let pixel_buffer = {
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

            Pixels::new(size.width, size.height, surface_texture)
                .expect("Error: create pixel buffer")
        };

        // Create the transformation matrix to project camera space onto NDC space
        let near_plane = 100.0;
        let far_plane = 1000.0;
        let fov = 90.0;
        let aspect_ratio = size.width as f32 / size.height as f32;

        let x_mul = (1.0 / f32::tan(fov / 2.0)) / aspect_ratio;
        let y_mul = 1.0 / f32::tan(fov / 2.0);
        let z1_mul = far_plane / (far_plane - near_plane);
        let z2_mul = -1.0 * (far_plane * near_plane) / (far_plane - near_plane);
        let projection_matrix = [
            [x_mul, 0.0, 0.0, 0.0],
            [0.0, y_mul, 0.0, 0.0],
            [0.0, 0.0, z1_mul, 1.0],
            [0.0, 0.0, z2_mul, 0.0],
        ];

        GraphicsWindow {
            window,
            size,
            pixel_buffer,
            projection_matrix,
        }
    }

    /// Resize the pixel buffer to match the window size
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.pixel_buffer
            .resize_surface(self.size.width, self.size.height);
        self.pixel_buffer
            .resize_buffer(self.size.width, self.size.height);

        // Recalculate the projection matrix
        let near_plane = 100.0;
        let far_plane = 1000.0;
        let fov = 90.0;
        let aspect_ratio = size.width as f32 / size.height as f32;

        let x_mul = (1.0 / f32::tan(fov / 2.0)) / aspect_ratio;
        let y_mul = 1.0 / f32::tan(fov / 2.0);
        let z1_mul = far_plane / (far_plane - near_plane);
        let z2_mul = -1.0 * (far_plane * near_plane) / (far_plane - near_plane);
        self.projection_matrix = [
            [x_mul, 0.0, 0.0, 0.0],
            [0.0, y_mul, 0.0, 0.0],
            [0.0, 0.0, z1_mul, 1.0],
            [0.0, 0.0, z2_mul, 0.0],
        ];
    }

    /// Clear the frame.
    pub fn clear(&mut self) {
        for i in self.pixel_buffer.get_frame().iter_mut() {
            *i = 0;
        }
    }

    /// Project a 3D line to NDC space.
    pub fn draw_line_3d(&mut self, line_camera_space: &Line) {
        // Convert the line from camera space to NDC space.
        let mut line_ndc_space = line_camera_space.transform(&self.projection_matrix);

        // Correct perspective due to the distance from the camera
        line_ndc_space[0] = line_ndc_space[0] / line_ndc_space[0].w;
        line_ndc_space[1] = line_ndc_space[1] / line_ndc_space[1].w;

        // Check coordinates are within the NDC space bounds.
        // TODO clipping function
        if line_ndc_space[0].x.abs() < 1.0
            && line_ndc_space[0].y.abs() < 1.0
            && line_ndc_space[0].z.abs() < 1.0
            && line_ndc_space[1].x.abs() < 1.0
            && line_ndc_space[1].y.abs() < 1.0
            && line_ndc_space[1].z.abs() < 1.0
        {
            // Convert from NDC space to screen space
            let line_screen_space = Line([
                Vertex::new(
                    ((line_ndc_space[0].x + 1.0) / 2.0) * self.size.width as f32,
                    ((line_ndc_space[0].y + 1.0) / 2.0) * self.size.height as f32,
                    0.0,
                    1.0,
                ),
                Vertex::new(
                    ((line_ndc_space[1].x + 1.0) / 2.0) * self.size.width as f32,
                    ((line_ndc_space[1].y + 1.0) / 2.0) * self.size.height as f32,
                    0.0,
                    1.0,
                ),
            ]);

            self.draw_line_2d(&line_screen_space);
        }
    }

    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line_2d(&mut self, line: &Line) {
        let (x1, y1) = (line[0].x as i32, line[0].y as i32);
        let (x2, y2) = (line[1].x as i32, line[1].y as i32);

        let mut dx = (x2 - x1).abs();
        let mut dy = (y2 - y1).abs();

        // Increment through the axis with the greatest difference between points
        if dx > dy {
            // Increment through the X axis but make sure we're incrementing not decrementing.
            // Swap the points if necessary and recalculate dx and dy.
            if x1 > x2 {
                let (x1, y1) = (line[1].x as i32, line[1].y as i32);
                let (x2, y2) = (line[0].x as i32, line[0].y as i32);
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
                let (x1, y1) = (line[1].x as i32, line[1].y as i32);
                let (x2, y2) = (line[0].x as i32, line[0].y as i32);
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

    /// Set a pixels colour via x and y coordinates with the origin in the bottom left corner.
    pub fn draw_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        // Figure out which 4 elements we need from the x and y coordinates then get a
        // pointer to the 4 elements which consitute the pixel.
        let element = {
            let y_invert = self.size.height - y;
            (((y_invert * self.size.width) + x) * 4) as usize
        };

        let pixel = self
            .pixel_buffer
            .get_frame()
            .get_mut(element..(element + 4))
            .unwrap();

        pixel.copy_from_slice(&colour);
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

    pub fn redraw(&self) {
        self.window.request_redraw();
    }
}
