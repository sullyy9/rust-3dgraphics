use crate::{
    mesh::{
        geometry::{Dim, Scalar},
        Transform,
    },
    rasterizer::EdgeTable,
};

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
    pub width: u32,
    pub height: u32,

    pixel_buffer: Pixels,
    zbuffer: Vec<Vec<f64>>,

    near_plane: f64,
    far_plane: f64,
    fov: f64,
    pub projection_matrix: Transform,
}
impl GraphicsWindow {
    ///
    /// Create a new graphics window.
    ///
    pub fn new(width: u32, height: u32, event_loop: &EventLoop<()>) -> GraphicsWindow {
        // Create the actual window
        let window = {
            let size = PhysicalSize::new(width, height);
            WindowBuilder::new()
                .with_title("3D Graphics")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(event_loop)
                .unwrap()
        };

        // Create a pixel buffer within the window
        let pixel_buffer = {
            let surface_texture = SurfaceTexture::new(width, height, &window);

            Pixels::new(width, height, surface_texture).expect("Error: create pixel buffer")
        };

        let zbuffer = vec![vec![0.0; (width + 1) as usize]; (height + 1) as usize];

        // Create the transformation matrix to project camera space onto NDC space
        let near_plane = 100.0;
        let far_plane = 1000.0;
        let fov = 45.0;

        let aspect_ratio = width as f64 / height as f64;
        let (near, far) = (near_plane, far_plane);

        let projection_matrix = Transform::builder()
            .scale_x(Scalar((1.0 / f64::tan(fov / 2.0)) / aspect_ratio))
            .scale_y(Scalar(1.0 / f64::tan(fov / 2.0)))
            .scale_z(Scalar(far / (far - near)))
            .translate_z(-(far * near) / (far - near))
            .build_projection();

        GraphicsWindow {
            window,
            width,
            height,
            pixel_buffer,
            zbuffer,
            near_plane,
            far_plane,
            fov,
            projection_matrix,
        }
    }

    ///
    /// Resize the pixel buffer to match the window size.
    ///
    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixel_buffer.resize_surface(width, height);
        self.pixel_buffer.resize_buffer(width, height);
        self.zbuffer = vec![vec![0.0; (width + 1) as usize]; (height + 1) as usize];

        // Recalculate the projection matrix
        self.projection_matrix = {
            let aspect_ratio = width as f64 / height as f64;
            let (near, far) = (self.near_plane, self.far_plane);

            Transform::builder()
                .scale_x(Scalar((1.0 / f64::tan(self.fov / 2.0)) / aspect_ratio))
                .scale_y(Scalar(1.0 / f64::tan(self.fov / 2.0)))
                .scale_z(Scalar(far / (far - near)))
                .translate_z(-(far * near) / (far - near))
                .build_projection()
        };

        self.width = width;
        self.height = height;
    }

    ///
    /// Clear the pixel buffer and z buffer.
    ///
    pub fn clear(&mut self) {
        for i in self.pixel_buffer.get_frame().iter_mut() {
            *i = 0;
        }
        for element in self.zbuffer.iter_mut() {
            for i in element {
                *i = 0.0;
            }
        }
    }

    ///
    /// Draw a polygon using rasterization.
    ///
    pub fn draw_polygon(&mut self, edge_table: &EdgeTable, style: DrawType) {
        // Calculate the green intensity from the z part of the polygons normal.
        // the Z normal will be between -1 and 1 with -1 facing the camera
        let colour = {
            let intensity = ((-edge_table.normal[Dim::Z] + 1.0) * 127.0) as u8;
            [0, intensity, 0, 255]
        };

        // Draw a rasterized polygon
        if style == DrawType::Fill || style == DrawType::Both {
            // Find the first and last elements we want to iterate between in the edge table.
            // We only want elements that will be within screen space.
            let (first, ystart) = if edge_table.ymin < 0 {
                (edge_table.ymin.abs() as usize, 0)
            } else {
                (0, edge_table.ymin)
            };
            let last = if edge_table.ymax > self.height as i32 {
                (self.height as i32 - edge_table.ymin) as usize
            } else {
                (edge_table.ymax - edge_table.ymin) as usize
            };

            for (i, edges) in edge_table.iter_between(first, last).enumerate() {
                let y = ystart + i as i32;
                match edges.get_edges() {
                    Ok(edges) => {
                        let xrange = {
                            let edge1 = edges[0].x.clamp(0, self.width as i32);
                            let edge2 = edges[1].x.clamp(0, self.width as i32);
                            edge1..edge2
                        };

                        // Find out how much Z changes for each X
                        let zstep = {
                            let dz = edges[1].z - edges[0].z;

                            let x1 = edges[0].x.clamp(0, self.width as i32);
                            let x2 = edges[1].x.clamp(0, self.width as i32);
                            let dx = x2 - x1;

                            dz as f64 / dx as f64
                        };
                        let mut z = edges[0].z as f64;

                        // interpolate X between the 2 edges.
                        for x in xrange {
                            if z > self.zbuffer[y as usize][x as usize] {
                                self.draw_pixel(x as u32, y as u32, colour);
                                self.zbuffer[y as usize][x as usize] = z;
                            }

                            z += zstep;
                        }
                    }
                    Err(_error) => {
                        println!("No edges for Y coordinate {}", y);
                    }
                };
            }
        }

        // Draw a wireframe polygon
        if style == DrawType::Wireframe || style == DrawType::Both {
            let mut y = edge_table.ymin;
            for edges in edge_table.iter() {
                for xzpair in edges.iter() {
                    if xzpair.x >= 0 && xzpair.x < self.width as i32 {
                        self.draw_pixel(xzpair.x as u32, y as u32, [255, 0, 0, 255]);
                    }
                }
                y += 1;
            }
        }
    }

    ///
    /// Set a pixels colour via x and y coordinates with the origin in the bottom left corner.
    ///
    pub fn draw_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        // Figure out which 4 elements we need from the x and y coordinates then get a
        // pointer to the 4 elements which consitute the pixel.
        let element = {
            let y_invert = self.height - (y + 1);
            (((y_invert * self.width) + x) * 4) as usize
        };

        let pixel = self
            .pixel_buffer
            .get_frame()
            .get_mut(element..(element + 4))
            .unwrap();

        pixel.copy_from_slice(&colour);
    }

    ///
    /// Render the pixel buffer to the screen.
    ///
    pub fn render(&mut self) {
        match self.pixel_buffer.render() {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to render pixel buffer")
            }
        }
    }

    ///
    /// Redraw the window.
    ///
    pub fn redraw(&self) {
        // Trgger a redraw event.
        self.window.request_redraw();
    }
}
