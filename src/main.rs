mod frame;
mod shapes;

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

    let cube = {
        let position = primitives::Point(100.0, 0.0, 210.0, 1.0);
        cube::Cube::new(position, 100)
    };

    for line in cube.get_lines().iter() {
        frame.draw_line_3d(&line);
    }

    let mut redraw = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // Handle close and resize requests
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => frame.resize(size.width, size.height),
                WindowEvent::ReceivedCharacter(_c) => redraw = true,
                _ => {}
            },
            Event::MainEventsCleared => {
                if redraw == true {
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                redraw = false;
                frame.render();
            }
            _ => (),
        }
    });
}
