use crate::primitives;
pub struct Cube {
    pub cube: [primitives::Triangle; 12],
    pub position: primitives::Point<i32>,
    pub vertex_length: u32,
}
impl Cube {
    /// Create a new cube.
    pub fn new(position: primitives::Point<i32>, vertex_length: u32) -> Cube {
        // Calculate the position offset of each point given the required centre position with
        // vertex length applied
        let vertex_length = vertex_length as i32;
        let point_offset: primitives::Point<i32> = [
            position[0] - (vertex_length / 2),
            position[1] - (vertex_length / 2),
            position[2] - (vertex_length / 2),
            position[3],
        ];

        // define the cube points. South is towards the screen
        let north_west_bottom: primitives::Point<i32> = [
            (0 * vertex_length) + point_offset[0],
            (0 * vertex_length) + point_offset[1],
            (1 * vertex_length) + point_offset[2],
            point_offset[3],
        ];
        let north_east_bottom: primitives::Point<i32> = [
            (1 * vertex_length) + point_offset[0],
            (0 * vertex_length) + point_offset[1],
            (1 * vertex_length) + point_offset[2],
            point_offset[3],
        ];
        let south_west_bottom: primitives::Point<i32> = [
            (0 * vertex_length) + point_offset[0],
            (0 * vertex_length) + point_offset[1],
            (0 * vertex_length) + point_offset[2],
            point_offset[3],
        ];
        let south_east_bottom: primitives::Point<i32> = [
            (1 * vertex_length) + point_offset[0],
            (0 * vertex_length) + point_offset[1],
            (0 * vertex_length) + point_offset[2],
            point_offset[3],
        ];

        let north_west_top: primitives::Point<i32> = [
            (0 * vertex_length) + point_offset[0],
            (1 * vertex_length) + point_offset[1],
            (1 * vertex_length) + point_offset[2],
            point_offset[3],
        ];
        let north_east_top: primitives::Point<i32> = [
            (1 * vertex_length) + point_offset[0],
            (1 * vertex_length) + point_offset[1],
            (1 * vertex_length) + point_offset[2],
            point_offset[3],
        ];
        let south_west_top: primitives::Point<i32> = [
            (0 * vertex_length) + point_offset[0],
            (1 * vertex_length) + point_offset[1],
            (0 * vertex_length) + point_offset[2],
            point_offset[3],
        ];
        let south_east_top: primitives::Point<i32> = [
            (1 * vertex_length) + point_offset[0],
            (1 * vertex_length) + point_offset[1],
            (0 * vertex_length) + point_offset[2],
            point_offset[3],
        ];

        let vertex_length = vertex_length as u32;
        Cube {
            cube: [
                // South face
                primitives::Triangle::new(south_west_bottom, south_west_top, south_east_top),
                primitives::Triangle::new(south_west_bottom, south_east_top, south_east_bottom),
                // East face
                primitives::Triangle::new(south_east_bottom, south_east_top, north_east_top),
                primitives::Triangle::new(south_east_bottom, north_east_top, north_east_bottom),
                // North face
                primitives::Triangle::new(north_east_bottom, north_east_top, north_west_top),
                primitives::Triangle::new(north_east_bottom, north_west_top, north_west_bottom),
                // West face
                primitives::Triangle::new(north_west_bottom, north_west_top, south_west_top),
                primitives::Triangle::new(north_west_bottom, south_west_top, south_west_bottom),
                // Top face
                primitives::Triangle::new(south_west_top, north_west_top, north_east_top),
                primitives::Triangle::new(south_west_top, north_east_top, south_east_top),
                // Bottom Face
                primitives::Triangle::new(north_west_bottom, south_west_bottom, south_east_bottom),
                primitives::Triangle::new(north_west_bottom, south_east_bottom, north_east_bottom),
            ],
            position,
            vertex_length,
        }
    }

    pub fn get_lines(&self) -> [primitives::Line; 36] {
        [
            self.cube[0].get_lines()[0],
            self.cube[0].get_lines()[1],
            self.cube[0].get_lines()[2],
            self.cube[1].get_lines()[0],
            self.cube[1].get_lines()[1],
            self.cube[1].get_lines()[2],
            self.cube[2].get_lines()[0],
            self.cube[2].get_lines()[1],
            self.cube[2].get_lines()[2],
            self.cube[3].get_lines()[0],
            self.cube[3].get_lines()[1],
            self.cube[3].get_lines()[2],
            self.cube[4].get_lines()[0],
            self.cube[4].get_lines()[1],
            self.cube[4].get_lines()[2],
            self.cube[5].get_lines()[0],
            self.cube[5].get_lines()[1],
            self.cube[5].get_lines()[2],
            self.cube[6].get_lines()[0],
            self.cube[6].get_lines()[1],
            self.cube[6].get_lines()[2],
            self.cube[7].get_lines()[0],
            self.cube[7].get_lines()[1],
            self.cube[7].get_lines()[2],
            self.cube[8].get_lines()[0],
            self.cube[8].get_lines()[1],
            self.cube[8].get_lines()[2],
            self.cube[9].get_lines()[0],
            self.cube[9].get_lines()[1],
            self.cube[9].get_lines()[2],
            self.cube[10].get_lines()[0],
            self.cube[10].get_lines()[1],
            self.cube[10].get_lines()[2],
            self.cube[11].get_lines()[0],
            self.cube[11].get_lines()[1],
            self.cube[11].get_lines()[2],
        ]
    }
}
