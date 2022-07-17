mod mesh;
mod physics;
mod rasterizer;
mod window;
mod world_object;

use std::{
    path::Path,
    time::{Duration, Instant},
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    mesh::{
        face_vertex::{Mesh, Visibility},
        geometry::{Dim, OrientationVector3D, Point, Vector},
        BBox, Pipeline, Scalar, Transform,
    },
    physics::PhysicalState,
    rasterizer::EdgeTable,
    window::{Colour, DrawType, GraphicsWindow},
};

fn main() -> ! {
    // Create the window
    let event_loop = EventLoop::new();
    let mut window = GraphicsWindow::new(960, 720, &event_loop);
    window.clear();

    //let cube = Mesh::new_cube(100.0);
    let cube = Mesh::new(Path::new("./resources/teapot.obj"));
    let mut physics = PhysicalState::new();
    physics.position = Point::new([0, 0, 400]);

    let mut cube_velocity = Vector::new([1, 1, 1]);

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
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => window.resize(size.width, size.height),
                WindowEvent::ReceivedCharacter(char) => match char {
                    ' ' => pause = !pause,
                    'n' => advance_frame = true,
                    _ => {}
                },
                _ => {}
            },

            // This event is triggered when all user events have been handled.
            // Decide whether to redraw the window at this time.
            Event::MainEventsCleared => {
                // Redraw if either:
                // Window is running and a new frame is due according to the framerate timer.
                // User has manualy requested a new frame.
                if !pause && Instant::now() >= time_of_next_frame {
                    let current_time = Instant::now();
                    time_of_current_frame = current_time;
                    time_of_next_frame = current_time + time_between_frames_min;

                    window.redraw();
                } else if advance_frame {
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
                if physics.position[Dim::X].abs() >= 200.0 {
                    cube_velocity[Dim::X] = -cube_velocity[Dim::X];
                }
                if physics.position[Dim::Y].abs() >= 150.0 {
                    cube_velocity[Dim::Y] = -cube_velocity[Dim::Y];
                }
                if physics.position[Dim::Z] >= 500.0 || physics.position[Dim::Z] <= 0.0 {
                    cube_velocity[Dim::Z] = -cube_velocity[Dim::Z];
                }

                // Move and rotate the mesh.
                physics.position.translate(&cube_velocity);
                physics.orientation += OrientationVector3D::new(1, 0.6, 3);

                let world_transform = Transform::builder()
                    .scale(Scalar(10.0))
                    .rotate_about_x(physics.orientation.vector().x)
                    .rotate_about_y(physics.orientation.vector().y)
                    .rotate_about_z(physics.orientation.vector().z)
                    .translate(physics.position.vector_from(&Point::new([0, 0, 0])))
                    .build_affine();

                let screen_transform = Transform::builder()
                    .translate_x(1.0)
                    .translate_y(1.0)
                    .scale_x(Scalar::from(window.width / 2))
                    .scale_y(Scalar::from(window.height / 2))
                    .scale_z(Scalar::from(-1000))
                    .translate_z(1000.0)
                    .build_affine();

                let ndc_bounds = BBox::new(Point::new([-1, -1, -1, -1]), Point::new([1, 1, 1, 1]));

                let screen_mesh = cube
                    .start_pipeline()
                    .transform(&world_transform)
                    .update_normals()
                    .transform(&window.projection_matrix)
                    .update_visibility(&ndc_bounds)
                    .transform(&screen_transform);

                let visible_polygons = screen_mesh.iter().filter(|polygon| match polygon.visible {
                    None => false,
                    Some(v) if *v == Visibility::None => false,
                    _ => true,
                });

                visible_polygons.for_each(|polygon| {
                    let colour = if let Some(normal) = polygon.normal {
                        let intensity = ((-normal[Dim::Z] + 1.0) * 127.0) as u8;
                        Colour([0, intensity, 0, u8::max_value()])
                    } else {
                        Colour([u8::max_value(), 0, 0, u8::max_value()])
                    };

                    window.draw_polygon(&EdgeTable::from(polygon), DrawType::Fill, &colour);
                });

                // Render the screen buffer.
                window.render();

                // Update the average frame draw time.
                let last_time = time_of_current_frame.elapsed().as_micros();
                draw_time_average.rotate_right(1);
                draw_time_average[0] = last_time;

                let mut average = 0;
                for time in draw_time_average.iter() {
                    average += time;
                }
                average /= 100;

                println!("average: {}, last: {}", average, last_time);
            }
            _ => (),
        }
    });
}
