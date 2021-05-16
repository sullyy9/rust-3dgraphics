mod mesh;
mod primitives;
mod rasterizer;
mod window;

use primitives as prim;
use rasterizer as rast;
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
    let mut cube_position = prim::Vertex::new(100.0, 0.0, 210.0, 1.0);
    let mut cube_orientation = prim::Vertex::new(0.0, 0.0, 0.0, 1.0);
    let mut cube = mesh::Mesh::new();
    cube.load_cube(100.0);

    // Current movement directions
    let mut move_right = true;
    let mut move_up = true;
    let mut move_back = true;

    let mut frame_timer = time::Instant::now();
    let mut draw_timer = time::Instant::now();
    let mut draw_time_average = [0; 100];

    // Controls
    let mut pause = false;
    let mut next_frame = false;

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
                WindowEvent::ReceivedCharacter(char) => match char {
                    ' ' => pause = !pause,
                    'n' => next_frame = true,
                    _ => {}
                },
                _ => {}
            },
            Event::MainEventsCleared => {
                if frame_timer.elapsed().as_millis() > 20 && pause == false {
                    frame_timer = time::Instant::now();
                    draw_timer = time::Instant::now();

                    window.redraw();
                } else if pause == true && next_frame == true {
                    window.redraw();
                    next_frame = false;
                }
            }
            Event::RedrawRequested(_) => {
                println!();
                println!("New frame---------------------");
                window.clear();

                // Adjust cube position
                if cube_position.x.abs() > 200.0 {
                    move_right = !move_right;
                }
                if move_right == true {
                    cube_position.x = cube_position.x + 1.0;
                } else {
                    cube_position.x = cube_position.x - 1.0;
                }

                if cube_position.y.abs() > 150.0 {
                    move_up = !move_up;
                }
                if move_up == true {
                    cube_position.y = cube_position.y + 1.0;
                } else {
                    cube_position.y = cube_position.y - 1.0;
                }

                if move_back == true {
                    cube_position.z = cube_position.z + 1.0;
                    if cube_position.z > 500.0 {
                        move_back = false;
                    }
                } else {
                    cube_position.z = cube_position.z - 1.0;
                    if cube_position.z < 300.0 {
                        move_back = true;
                    }
                }

                // Adjust cube orientation
                cube_orientation.x = cube_orientation.x + 0.5;
                if cube_orientation.x > 180.0 {
                    cube_orientation.x = -180.0;
                }

                cube_orientation.y = cube_orientation.y + 0.6;
                if cube_orientation.y > 180.0 {
                    cube_orientation.y = -180.0;
                }

                cube_orientation.z = cube_orientation.z + 0.3;
                if cube_orientation.z > 180.0 {
                    cube_orientation.z = -180.0;
                }

                // Transform the cube
                cube_position = prim::Vertex::new(0.0, 0.0, 200.0, 1.0);
                cube.position = cube_position;
                cube.rotate(cube_orientation);

                // Run through the pipeline with a copy of the cube
                let mut cube_pipe = cube.clone();

                cube_pipe.find_normals();
                cube_pipe.project_to_ndc(&window.projection_matrix);
                cube_pipe.polygons_in_view();
                cube_pipe.project_to_screen(window.size.width as f32, window.size.height as f32);

                for index_polygon in cube_pipe.visible_polygons.iter() {
                    let polygon = cube_pipe.get_polygon_owned(index_polygon);
                    let edge_table = rast::EdgeTable::new(polygon);
                    // println!("draw polygon--------");
                    // println!("p1   {0: <10}, {1: <10}, {2: <10}", polygon.p1.x, polygon.p1.y, polygon.p1.z);
                    // println!("p2   {0: <10}, {1: <10}, {2: <10}", polygon.p2.x, polygon.p2.y, polygon.p2.z);
                    // println!("p3   {0: <10}, {1: <10}, {2: <10}", polygon.p3.x, polygon.p3.y, polygon.p3.z);
                    // println!("norm {0: <10}, {1: <10}, {2: <10}", polygon.normal.x, polygon.normal.y, polygon.normal.z);
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
