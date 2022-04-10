mod mesh;
mod primitives;
mod matrix;
mod rasterizer;
mod window;
mod physics;

use physics::PhysicalObject;
use rasterizer as rast;

use std::time::{Duration, Instant};

use window::GraphicsWindow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() -> ! {
    // Create the window
    let event_loop = EventLoop::new();
    let mut window = GraphicsWindow::new(960, 720, &event_loop);
    window.clear();

    // Build a mesh in the form of a cube.
    // Set it's initial position and velocities so that it moves around the screen.
    let mut cube = mesh::Mesh::new();
    cube.load_cube(100.0);
    cube.set_absolute_position(0.0, 0.0, 400.0);
    let mut cube_velocity_x = 1.0;
    let mut cube_velocity_y = 1.0;
    let mut cube_velocity_z = 1.0;

    // Set controls for pausing and manually advancing each frame.
    let mut pause = false;
    let mut advance_frame = false;

    // Set up a timers to limit and measure frame rate.
    // Aim for 15ms minimum between frames. Equivilent to 66.6FPS.
    let mut time_of_current_frame = Instant::now();
    let mut time_of_next_frame = Instant::now();
    let time_between_frames_min = Duration::from_millis(15);
    let mut draw_time_average = [0; 100];

    // Start the main loop. I think this creates a new thread to run the interior code.
    event_loop.run(move |event, _, control_flow| {
        // This controls how the thread runs the code. In poll mode, it will loop through the code.
        *control_flow = ControlFlow::Poll;

        match event {
            // Handle any event triggered by the user.
            // E.g resizing the window, key presses, etc.
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                // User wants to close the window.
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                // User has resized the window.
                WindowEvent::Resized(size) => window.resize(size.width, size.height),

                // User has pressed a key.
                WindowEvent::ReceivedCharacter(char) => match char {
                    ' ' => pause = !pause,
                    'n' => advance_frame = true,
                    _ => {}
                },

                // Anything else.
                _ => {}
            },

            // This event is triggered when all user events have been handled.
            // Decide whether to redraw the window at this time.
            Event::MainEventsCleared => {
                // Redraw if either:
                // Window is running and a new frame is due according to the framerate timer.
                // User has manualy requested a new frame.
                if pause == false && Instant::now() >= time_of_next_frame {
                    let current_time = Instant::now();
                    time_of_current_frame = current_time;
                    time_of_next_frame = current_time + time_between_frames_min;

                    window.redraw();
                } else if advance_frame == true {
                    advance_frame = false;

                    window.redraw();
                }
            }

            // This event is triggered when a new frame needs to be drawn.
            // Move any objects and draw them.
            Event::RedrawRequested(_) => {
                println!();
                println!("New frame---------------------");
                window.clear();

                // Flip the direction of travel along an axis if its position along that axis has reached a limit.
                if cube.physical_state.position.x.abs() >= 200.0 {
                    cube_velocity_x = -cube_velocity_x;
                }
                if cube.physical_state.position.y.abs() >= 150.0 {
                    cube_velocity_y = -cube_velocity_y;
                }
                if cube.physical_state.position.z >= 500.0 || cube.physical_state.position.z <= 0.0 {
                    cube_velocity_z = -cube_velocity_z;
                }

                // Move and rotate the mesh.
                cube.set_relative_position(cube_velocity_x, cube_velocity_y, cube_velocity_z);
                cube.set_relative_orientation(0.5, 0.6, 0.3);

                // Get a copy of the cube that's been run through the pipeline.
                // This copy will be in NDC space.
                let cube_pipe = cube.run_pipeline(
                    &window.projection_matrix,
                    [window.width as f64, window.height as f64],
                );

                // Generate an edge table for every polygon in the mesh and draw it to the screen buffer.
                for polygon in cube_pipe.iter_visible_polygons() {
                    let edge_table = rast::EdgeTable::new(polygon);
                    window.draw_polygon(&edge_table, window::DrawType::Fill);
                }

                // Render the screen buffer.
                window.render();

                // Update the average frame draw time.
                let last_time = time_of_current_frame.elapsed().as_micros();
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
