use crate::primitives as prim;
use crate::rasterizer as rast;

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
    pub projection_matrix: prim::TransformMatrix,
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
        let projection_matrix = prim::TransformMatrix([
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
        self.projection_matrix = prim::TransformMatrix([
            [x_mul, 0.0, 0.0, 0.0],
            [0.0, y_mul, 0.0, 0.0],
            [0.0, 0.0, z1_mul, 1.0],
            [0.0, 0.0, z2_mul, 0.0],
        ]);
    }

    /// Clear the pixel buffer.
    pub fn clear(&mut self) {
        for i in self.pixel_buffer.get_frame().iter_mut() {
            *i = 0;
        }
    }

    /// Draw a polygon using rasterization, as a wireframe or both.
    pub fn draw_polygon(&mut self, edge_table: &rast::EdgeTable, style: DrawType) {
        // Calculate the green intensity from the z part of the polygons normal.
        // the Z normal will be between -1 and 1 with -1 facing the camera
        let mut colour = {
            let intensity = ((-edge_table.normal.z + 1.0) * 127.0) as u8;
            [0, intensity, 0, 0]
        };

        // Draw a rasterized polygon
        if style == DrawType::Fill || style == DrawType::Both {
            let mut y = edge_table.ymin;
            for edges in edge_table.iter() {
                match edges.get_edges() {
                    Ok(edges) => {
                        let xrange = edges[0].x..edges[1].x;

                        // Find out how much Z changes for each X
                        let zstep = {
                            let dz = edges[1].z - edges[0].z;
                            let dx = edges[1].x - edges[0].x;
                            dz as f32 / dx as f32
                        };
                        let mut z = edges[0].z as f32;

                        // interpolate X between the 2 edges.
                        for x in xrange {
                            colour[3] = z as u8;
                            self.draw_pixel(x as u32, y as u32, colour);

                            z = z + zstep;
                        }
                    },
                    Err(_error) => {
                        println!("No edges for Y coordinate {}", y);
                    },
                };
                y = y + 1;
            }
        }

        // Draw a wireframe polygon
        if style == DrawType::Wireframe || style == DrawType::Both {
            let mut y = edge_table.ymin;
            for edges in edge_table.iter() {
                for xzpair in edges.iter() {
                    if xzpair.x >= 0 && xzpair.x < self.size.width as i32 {
                        self.draw_pixel(xzpair.x as u32, y as u32, [255, 0, 0, 255]);
                    }
                }
                y = y + 1;
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
        if colour[3] > pixel[3] {
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
