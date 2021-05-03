mod frame;
mod shapes;

use std::time;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use shapes::{cube, primitives};

const RESOLUTION_WIDTH: u32 = 960;
const RESOLUTION_HEIGHT: u32 = 720;

fn main() {
    let window_size = LogicalSize::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("3D Graphics")
        .with_inner_size(window_size)
        .with_min_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let mut frame = frame::Frame::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT, &window);
    frame.clear();

    let mut cube_position = primitives::Point(100.0, 0.0, 210.0, 1.0);
    let mut cube = cube::Cube::new(cube_position, 100);

    let mut move_right = true;
    let mut move_up = true;
    let mut move_back = true;

    let mut timer = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            // Handle close and resize requests
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => frame.resize(size.width, size.height),
                // WindowEvent::ReceivedCharacter(_c) => redraw = true,
                _ => {}
            },
            Event::MainEventsCleared => {
                if timer.elapsed().as_millis() > 20 {
                    window.request_redraw();
                    timer = time::Instant::now();
                }
            }
            Event::RedrawRequested(_) => {
                frame.clear();

                if move_right == true {
                    cube_position.0 = cube_position.0 + 2.0;
                    if cube_position.0 > 200.0 {
                        move_right = false;
                    }
                } else {
                    cube_position.0 = cube_position.0 - 2.0;
                    if cube_position.0 < -200.0 {
                        move_right = true;
                    }
                }

                if move_up == true {
                    cube_position.1 = cube_position.1 + 2.0;
                    if cube_position.1 > 150.0 {
                        move_up = false;
                    }
                } else {
                    cube_position.1 = cube_position.1 - 2.0;
                    if cube_position.1 < -150.0 {
                        move_up = true;
                    }
                }

                if move_back == true {
                    cube_position.2 = cube_position.2 + 2.0;
                    if cube_position.2 > 400.0 {
                        move_back = false;
                    }
                } else {
                    cube_position.2 = cube_position.2 - 2.0;
                    if cube_position.2 < 200.0 {
                        move_back = true;
                    }
                }

                cube.translate(&cube_position);
                

                for line in cube.get_lines().iter() {
                    frame.draw_line_3d(&line);
                }

                frame.render();
            }
            _ => (),
        }
    });
}
