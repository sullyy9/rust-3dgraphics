mod shapes;
mod window;

use crate::shapes::{
    cube,
    primitives::{Line, Vertex},
};
use std::time;
use window::GraphicsWindow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

const RESOLUTION_WIDTH: u32 = 960;
const RESOLUTION_HEIGHT: u32 = 720;

fn main() {
    // Create the window
    let event_loop = EventLoop::new();
    let mut window = GraphicsWindow::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT, &event_loop);
    window.clear();

    // Create a cube
    let mut cube_position = Vertex::new(100.0, 0.0, 210.0, 1.0);
    let mut cube_orientation = Vertex::new(0.0, 0.0, 0.0, 1.0);
    let mut cube = cube::Cube::new(100.0, cube_position, cube_orientation);

    // Current movement directions
    let mut move_right = true;
    let mut move_up = true;
    let mut move_back = true;

    let mut frame_timer = time::Instant::now();
    let mut draw_timer = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            // Handle close and resize requests
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => window.resize(size),
                // WindowEvent::ReceivedCharacter(_c) => redraw = true,
                _ => {}
            },
            Event::MainEventsCleared => {
                if frame_timer.elapsed().as_millis() > 20 {
                    frame_timer = time::Instant::now();
                    draw_timer = time::Instant::now();

                    window.redraw();
                }
            }
            Event::RedrawRequested(_) => {
                window.clear();

                if move_right == true {
                    cube_position.x = cube_position.x + 2.0;
                    if cube_position.x > 200.0 {
                        move_right = false;
                    }
                } else {
                    cube_position.x = cube_position.x - 2.0;
                    if cube_position.x < -200.0 {
                        move_right = true;
                    }
                }

                if move_up == true {
                    cube_position.y = cube_position.y + 2.0;
                    if cube_position.y > 150.0 {
                        move_up = false;
                    }
                } else {
                    cube_position.y = cube_position.y - 2.0;
                    if cube_position.y < -150.0 {
                        move_up = true;
                    }
                }

                if move_back == true {
                    cube_position.z = cube_position.z + 2.0;
                    if cube_position.z > 800.0 {
                        move_back = false;
                    }
                } else {
                    cube_position.z = cube_position.z - 2.0;
                    if cube_position.z < 200.0 {
                        move_back = true;
                    }
                }

                cube_orientation = cube_orientation + 1.0;
                if cube_orientation.x > 180.0 {
                    cube_orientation = Vertex::new(-180.0, -180.0, -180.0, 1.0);
                }
                cube_orientation.w = 1.0;

                cube.position = cube_position;
                cube.rotate(cube_orientation);

                for face in cube.faces.iter() {
                    for line in face.get_lines().iter() {
                        window.draw_line_3d(&Line([
                            line[0] + cube.position,
                            line[1] + cube.position,
                        ]));
                    }
                }

                window.render();

                println!(
                    "draw time: {}.{}us",
                    draw_timer.elapsed().as_micros(),
                    draw_timer.elapsed().as_nanos()
                );
            }
            _ => (),
        }
    });
}
