use frame::Frame;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use rand::Rng;

mod frame;
mod shape;

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

    let mut frame = Frame::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT, &window);
    frame.draw_border();

    let mut line = {
        let point1 = shape::Point { x: 700, y: 50 };
        let point2 = shape::Point { x: 100, y: 600 };
        shape::Line::new(point1, point2)
    };

    //frame.draw_pixel(3, 100, [0, 255, 0, 255]);
    frame.draw_line(&line);

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
                randomize_line(&mut line);
                frame.draw_line(&line);
                frame.render();
            }
            _ => (),
        }
    });
}

fn randomize_line(line: &mut shape::Line) {
    let x1 = rand::thread_rng().gen_range(3..(RESOLUTION_WIDTH-2));
    let x2 = rand::thread_rng().gen_range(3..(RESOLUTION_WIDTH-2));
    let y1 = rand::thread_rng().gen_range(3..(RESOLUTION_HEIGHT-2));
    let y2 = rand::thread_rng().gen_range(3..(RESOLUTION_HEIGHT-2));

    println!("Random line: ({}, {}) -> ({}, {})", x1, y1, x2, y2);

    line.point1.x = x1;
    line.point1.y = y1;
    line.point2.x = x2;
    line.point2.y = y2;
}