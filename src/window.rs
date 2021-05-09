use crate::shapes::primitives::TransformationMatrix;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

type Colour = [u8; 4];

#[derive(PartialEq)]
pub enum DrawType {
    Wireframe,
    Fill,
    Both,
}

pub struct GraphicsWindow {
    window: Window,
    pub size: PhysicalSize<u32>,

    pub pixel_buffer: Pixels,

    near_plane: f32,
    far_plane: f32,
    fov: f32,
    pub projection_matrix: TransformationMatrix,
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
        let projection_matrix = TransformationMatrix([
            [x_mul, 0.0, 0.0, 0.0],
            [0.0, y_mul, 0.0, 0.0],
            [0.0, 0.0, z1_mul, 1.0],
            [0.0, 0.0, z2_mul, 0.0],
        ]);

        GraphicsWindow {
            window,
            size,
            pixel_buffer,
            near_plane,
            far_plane,
            fov,
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
        let aspect_ratio = size.width as f32 / size.height as f32;

        let x_mul = (1.0 / f32::tan(self.fov / 2.0)) / aspect_ratio;
        let y_mul = 1.0 / f32::tan(self.fov / 2.0);
        let z1_mul = self.far_plane / (self.far_plane - self.near_plane);
        let z2_mul = -1.0 * (self.far_plane * self.near_plane) / (self.far_plane - self.near_plane);
        self.projection_matrix = TransformationMatrix([
            [x_mul, 0.0, 0.0, 0.0],
            [0.0, y_mul, 0.0, 0.0],
            [0.0, 0.0, z1_mul, 1.0],
            [0.0, 0.0, z2_mul, 0.0],
        ]);
    }

    /// Clear the frame.
    pub fn clear(&mut self) {
        for i in self.pixel_buffer.get_frame().iter_mut() {
            *i = 0;
        }
    }

    pub fn draw_polygon(
        &mut self,
        edge_matrix: &Vec<Vec<i32>>,
        y_offset: u32,
        draw_type: DrawType,
    ) {
        let mut y_coord = y_offset;

        if draw_type == DrawType::Fill || draw_type == DrawType::Both {
            for y in edge_matrix.iter() {
                if y.len() >= 2 {
                    for x in y[0]..(*y.last().unwrap()) {
                        if x >= 0 && x < self.size.width as i32 {
                            self.draw_pixel(x as u32, y_coord, [0, 200, 0, 254]);
                        }
                    }
                }
                y_coord = y_coord + 1;
            }
        }

        // Draw wireframe
        y_coord = y_offset;
        if draw_type == DrawType::Wireframe || draw_type == DrawType::Both {
            for y in edge_matrix.iter() {
                for x in y.iter() {
                    if *x >= 0 && *x < self.size.width as i32 {
                        self.draw_pixel(*x as u32, y_coord, [0, 255, 0, 255]);
                    }
                }
                y_coord = y_coord + 1;
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

        // Using the alpha channel as a z buffer
        if pixel[3] < colour[3] {
            pixel.copy_from_slice(&colour);
        }
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
