use std::mem::swap;

use crate::shapes::{
    cube::Cube,
    primitives::{TransformationMatrix, Triangle},
};
use winit::dpi::PhysicalSize;

/// Project a mesh object from camera space to NDC space.
/// TODO implement iter for triangles.
pub fn project_to_ndc_space(mut cube: Cube, projection_matrix: &TransformationMatrix) -> Cube {
    for polygon in cube.polygons.iter_mut() {
        // Convert the polygons points from camera space to NDC space.
        polygon.p1 = polygon.p1 + cube.position;
        polygon.p2 = polygon.p2 + cube.position;
        polygon.p3 = polygon.p3 + cube.position;

        polygon.p1 = polygon.p1 * (*projection_matrix);
        polygon.p2 = polygon.p2 * (*projection_matrix);
        polygon.p3 = polygon.p3 * (*projection_matrix);

        // Correct perspective due to the distance from the camera
        polygon.p1 = polygon.p1 / polygon.p1.w;
        polygon.p2 = polygon.p2 / polygon.p2.w;
        polygon.p3 = polygon.p3 / polygon.p3.w;
    }

    cube
}

/// Take a mesh and clip any polygons that won't be drawn. Transform the rest to screen space.
/// TODO implement iter for triangles.
pub fn get_polygons_to_draw(cube: &Cube, screen_size: &PhysicalSize<u32>) -> Vec<Triangle> {
    let mut polygon_list: Vec<Triangle> = Vec::new();

    for polygon in cube.polygons.iter() {
        // Clip any polgon that has all of its points outside of NDC space.
        // If just one point is within NDC space we need to draw that part of it.
        if (polygon.p1.x.abs() < 1.0 && polygon.p1.y.abs() < 1.0 && polygon.p1.z.abs() < 1.0)
            && (polygon.p2.x.abs() < 1.0 && polygon.p2.y.abs() < 1.0 && polygon.p2.z.abs() < 1.0)
            && (polygon.p3.x.abs() < 1.0 && polygon.p3.y.abs() < 1.0 && polygon.p3.z.abs() < 1.0)
        {
            let mut polygon_screen = *polygon;
            polygon_screen.p1.x = ((polygon.p1.x + 1.0) / 2.0) * screen_size.width as f32;
            polygon_screen.p1.y = ((polygon.p1.y + 1.0) / 2.0) * screen_size.height as f32;

            polygon_screen.p2.x = ((polygon.p2.x + 1.0) / 2.0) * screen_size.width as f32;
            polygon_screen.p2.y = ((polygon.p2.y + 1.0) / 2.0) * screen_size.height as f32;

            polygon_screen.p3.x = ((polygon.p3.x + 1.0) / 2.0) * screen_size.width as f32;
            polygon_screen.p3.y = ((polygon.p3.y + 1.0) / 2.0) * screen_size.height as f32;

            polygon_list.push(polygon_screen);
        }
    }

    polygon_list
}

/// Rasterize a screen space polygon. Return a matrix containing the coordinates
/// of every edge and the y offset of the matrix.
pub fn rasterize_polygon(mut polygon: Triangle, screen_size: &PhysicalSize<u32>) -> (Vec<Vec<i32>>, i32) {
    // array of each edge in X for each Y coordinate
    // edge_matrix[y][x]. Each Y element should have at least 2 X values
    let mut edge_matrix: Vec<Vec<i32>> = Vec::new();

    // Add enough Y elements to the edge_matrix vector. Clip Y to within the screen.
    let (mut y_min, mut y_max) = min_max(
        polygon.p1.y as i32,
        polygon.p2.y as i32,
        polygon.p3.y as i32,
    );

    if y_max >= screen_size.height as i32 {
        y_max = (screen_size.height - 1) as i32;
    }
    if y_min < 0 {
        y_min = 0;
    }

    for _ in y_min..(y_max + 1) {
        edge_matrix.push(Vec::new());
    }

    // Order the polygon's verticies so that we draw the left-most lines first
    if polygon.p2.x < polygon.p1.x && polygon.p2.x < polygon.p3.x {
        swap(&mut polygon.p1, &mut polygon.p2);
    } else if polygon.p3.x < polygon.p1.x && polygon.p3.x < polygon.p2.x {
        swap(&mut polygon.p1, &mut polygon.p3);
    }
    if polygon.p3.x < polygon.p2.x {
        swap(&mut polygon.p2, &mut polygon.p3);
    }

    // Draw the polygon's lines into the edge_matrix using brezenham's algorithm
    draw_line(
        polygon.p1.x as i32,
        polygon.p1.y as i32,
        polygon.p2.x as i32,
        polygon.p2.y as i32,
        &mut edge_matrix,
        y_min,
    );

    draw_line(
        polygon.p1.x as i32,
        polygon.p1.y as i32,
        polygon.p3.x as i32,
        polygon.p3.y as i32,
        &mut edge_matrix,
        y_min,
    );

    draw_line(
        polygon.p2.x as i32,
        polygon.p2.y as i32,
        polygon.p3.x as i32,
        polygon.p3.y as i32,
        &mut edge_matrix,
        y_min,
    );

    (edge_matrix, y_min)
}

/// Draw a line into the edge_matrix using brezenhams algorithm
fn draw_line(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    edge_matrix: &mut Vec<Vec<i32>>,
    y_offset: i32,
) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    // Increment through the axis with the greatest difference between points
    if dx > dy {
        // Increment through the X axis but make sure we're incrementing not decrementing.
        // Swap the points if necessary.
        if x1 > x2 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }

        // Decide if we need to increment or decrement y.
        let inc_dec = if y1 > y2 { -1 } else { 1 };

        let mut accumulated_y = (2 * dy) - dx;
        let mut y = y1;

        // Draw the line, incrementing/decrementing y as the accumulated change in y crosses the
        // threshold of what ammounts to a pixel.
        for x in x1..x2 {
            edge_matrix[(y - y_offset) as usize].push(x);

            if accumulated_y > 0 {
                y = y + inc_dec;
                accumulated_y = accumulated_y - (2 * dx);
            }
            accumulated_y = accumulated_y + (2 * dy);
        }
    } else {
        // Increment through the Y axis but make sure we're incrementing not decrementing.
        // Swap the points if necessary.
        if y1 > y2 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }

        // Decide if we need to increment or decrement x.
        let inc_dec = if x1 > x2 { -1 } else { 1 };

        let mut accumulated_x = (2 * dx) - dy;
        let mut x = x1;

        // Draw the line, incrementing/decrementing x as the accumulated change in x crosses the
        // threshold of what ammounts to a pixel.
        for y in y1..y2 {
            edge_matrix[(y - y_offset) as usize].push(x);

            if accumulated_x > 0 {
                x = x + inc_dec;
                accumulated_x = accumulated_x - (2 * dy);
            }
            accumulated_x = accumulated_x + (2 * dx);
        }
    }
}

/// Find the minimum and maximum of 3 f32 variables.
/// Return -> (min, max)
fn min_max(val1: i32, val2: i32, val3: i32) -> (i32, i32) {
    let mut result = if val1 < val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };

    if val3 < result.0 {
        result.0 = val3;
    } else if val3 > result.1 {
        result.1 = val3;
    }

    result
}
