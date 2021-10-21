mod mesh;
mod primitives;
mod rasterizer;
mod window;

use rasterizer as rast;

use std::time::{Duration, Instant};

use window::GraphicsWindow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
    // Create the window
    let event_loop = EventLoop::new();
    let mut window = GraphicsWindow::new(960, 720, &event_loop);
    window.clear();

    // Create a cube.
    let mut cube = mesh::Mesh::new();
    cube.load_cube(100.0);
    cube.abs_position(0.0, 0.0, 400.0);
    let mut cube_velocity_x = 1.0;
    let mut cube_velocity_y = 1.0;
    let mut cube_velocity_z = 1.0;

    // Timers.
    let mut draw_timer = Instant::now();
    let mut draw_time_average = [0; 100];

    // Controls.
    let mut pause = false;
    let mut advance_frame = false;

    // Frame rate.
    let mut next_frame = Instant::now();
    let frame_rate = Duration::from_millis(15);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            // Handle close and resize requests
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => window.resize(size.width, size.height),
                WindowEvent::ReceivedCharacter(char) => match char {
                    ' ' => pause = !pause,
                    'n' => advance_frame = true,
                    _ => {}
                },
                _ => {}
            },
            Event::MainEventsCleared => {
                if pause == false && Instant::now() >= next_frame {
                    let now = Instant::now();
                    draw_timer = now;
                    next_frame = now + frame_rate;

                    window.redraw();
                } else if advance_frame == true {
                    advance_frame = false;

                    window.redraw();
                }
            }
            Event::RedrawRequested(_) => {
                println!();
                println!("New frame---------------------");
                window.clear();

                // Adjust cube velocity
                if cube.position.x.abs() >= 200.0 {
                    cube_velocity_x = -cube_velocity_x;
                }
                if cube.position.y.abs() >= 150.0 {
                    cube_velocity_y = -cube_velocity_y;
                }
                if cube.position.z >= 500.0 || cube.position.z <= 0.0 {
                    cube_velocity_z = -cube_velocity_z;
                }

                // Perform transformations on the mesh.
                cube.rel_position(cube_velocity_x, cube_velocity_y, cube_velocity_z);
                cube.rel_orientation(0.5, 0.6, 0.3);

                // Get a copy of the cube that's been run through the pipeline.
                let cube_pipe = cube.run_pipeline(
                    &window.projection_matrix,
                    [window.width as f32, window.height as f32],
                );

                // Rasterize every visible polygon and draw it to the screen buffer.
                for polygon in cube_pipe.iter_visible_polygons() {
                    let edge_table = rast::EdgeTable::new(polygon);
                    window.draw_polygon(&edge_table, window::DrawType::Fill);
                }

                window.render();

                // Calculate draw time
                let last_time = draw_timer.elapsed().as_micros();
                draw_time_average.rotate_right(1);
                draw_time_average[0] = last_time;

                let mut average = 0;
                for time in draw_time_average.iter() {
                    average = average + time;
                }
                average = average / 100;

                println!("average: {}, last: {}", average, last_time);
            }
            _ => (),
        }
    });
}
