use pixels::{Pixels, SurfaceTexture};

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder}, platform::unix::WindowBuilderExtUnix,
};

use crate::{buffer::PixelBuffer, geometry::Scalar, mesh::Transform};

pub struct Colour(pub [u8; 4]);

impl From<[u8; 4]> for Colour {
    fn from(colour: [u8; 4]) -> Self {
        Colour(colour)
    }
}

impl<'a> Colour {
    pub fn as_slice(&'a self) -> &'a [u8] {
        &self.0
    }
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
                .with_decorations(true)
                .build(event_loop)
                .unwrap()
        };

        // Create a pixel buffer within the window
        let pixel_buffer = {
            let surface_texture = SurfaceTexture::new(width, height, &window);

            match Pixels::new(width, height, surface_texture) {
                Ok(result) => result,
                Err(error) => panic!("Failed to create pixel buffer: {}", error),
            }
        };

        let zbuffer = vec![vec![0.0; (width + 1) as usize]; (height + 1) as usize];

        // Create the transformation matrix to project camera space onto NDC space
        let near_plane = 1.0;
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

    pub fn draw_buffer(&mut self, pixel_buffer: PixelBuffer) {
        self.pixel_buffer
            .get_frame()
            .clone_from_slice(pixel_buffer.get_data());
    }

    ///
    /// Set a pixels colour via x and y coordinates with the origin in the bottom left corner.
    ///
    pub fn draw_pixel(&mut self, x: u32, y: u32, colour: &Colour) {
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

        pixel.copy_from_slice(colour.as_slice());
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
