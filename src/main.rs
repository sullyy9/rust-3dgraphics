mod camera;
mod mesh;
mod rasterizer;
mod window;
mod world_object;

use std::{
    path::Path,
    time::{Duration, Instant},
};

use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    camera::Camera,
    mesh::{
        geometry::{Dim, OrientationVector3D, Point, Vector},
        BBox, Mesh, Pipeline, Renderable, Scalar, Transform, Visibility,
    },
    rasterizer::EdgeTable,
    window::{Colour, DrawType, GraphicsWindow},
    world_object::WorldObject,
};

#[derive(Default)]
struct CameraControls {
    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,

    look_left: bool,
    look_right: bool,
    look_up: bool,
    look_down: bool,
    look_cw: bool,
    look_acw: bool,
}

fn main() -> ! {
    // Create the window
    let event_loop = EventLoop::new();
    let mut window = GraphicsWindow::new(960, 720, &event_loop);
    window.clear();

    //let cube = Mesh::new_cube(100.0);
    let mut teapot = WorldObject::new(Mesh::new(Path::new("./resources/teapot.obj")));
    teapot.position = Point::new([0, 0, 400]);

    let mut camera = Camera::new(Point::new([0, 0, 0]));
    let mut controls = CameraControls::default();

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
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    ..
                } => {
                    if let Some(key) = input.virtual_keycode {
                        let state = input.state;
                        use ElementState::*;
                        use VirtualKeyCode as KeyCode;
                        match key {
                            KeyCode::Escape => *control_flow = ControlFlow::Exit,
                            KeyCode::Space => pause = !pause,
                            KeyCode::N => advance_frame = true,

                            KeyCode::W if state == Pressed => controls.move_forward = true,
                            KeyCode::W if state == Released => controls.move_forward = false,
                            KeyCode::S if state == Pressed => controls.move_backward = true,
                            KeyCode::S if state == Released => controls.move_backward = false,

                            KeyCode::A if state == Pressed => controls.move_left = true,
                            KeyCode::A if state == Released => controls.move_left = false,
                            KeyCode::D if state == Pressed => controls.move_right = true,
                            KeyCode::D if state == Released => controls.move_right = false,

                            KeyCode::LShift if state == Pressed => controls.move_up = true,
                            KeyCode::LShift if state == Released => controls.move_up = false,
                            KeyCode::LControl if state == Pressed => controls.move_down = true,
                            KeyCode::LControl if state == Released => controls.move_down = false,

                            KeyCode::Up if state == Pressed => controls.look_up = true,
                            KeyCode::Up if state == Released => controls.look_up = false,
                            KeyCode::Down if state == Pressed => controls.look_down = true,
                            KeyCode::Down if state == Released => controls.look_down = false,

                            KeyCode::Left if state == Pressed => controls.look_left = true,
                            KeyCode::Left if state == Released => controls.look_left = false,
                            KeyCode::Right if state == Pressed => controls.look_right = true,
                            KeyCode::Right if state == Released => controls.look_right = false,

                            KeyCode::E if state == Pressed => controls.look_cw = true,
                            KeyCode::E if state == Released => controls.look_cw = false,
                            KeyCode::Q if state == Pressed => controls.look_acw = true,
                            KeyCode::Q if state == Released => controls.look_acw = false,

                            _ => {}
                        }
                    }
                }
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

                let map_move = |positive, negative| {
                    let mut movement = 0.0;
                    if positive {
                        movement += 1.0;
                    }
                    if negative {
                        movement -= 1.0;
                    }
                    movement
                };
                camera.position[Dim::X] += map_move(controls.move_right, controls.move_left);
                camera.position[Dim::Y] += map_move(controls.move_up, controls.move_down);
                camera.position[Dim::Z] += map_move(controls.move_forward, controls.move_backward);
                camera.orientation.x += map_move(controls.look_up, controls.look_down);
                camera.orientation.y += map_move(controls.look_left, controls.look_right);
                camera.orientation.z += map_move(controls.look_cw, controls.look_acw);

                let world_transform = Transform::builder()
                    .scale(Scalar(10.0))
                    .rotate_about_x(teapot.orientation.vector().x)
                    .rotate_about_y(teapot.orientation.vector().y)
                    .rotate_about_z(teapot.orientation.vector().z)
                    .translate(teapot.position.vector_from(&Point::new([0, 0, 0])))
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

                let screen_mesh = teapot
                    .mesh
                    .start_pipeline()
                    .transform(&world_transform)
                    .transform(&camera.view_transform())
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
