use crate::util::Vertex;

struct Cube {
    pub pos: (i8, i8, i8)
}

pub struct Scene {
    cubes: Vec<Cube>,
}

const CUBE_SIZE: i8 = 1;
const CUBE_COLOR: [i8; 4] = [44, 62, 80, 1];

impl Scene {
    pub fn new() -> Self {
        Scene {
            cubes: vec![],
        }
    }
    pub fn add_cube(&mut self, x:i8, y: i8, z: i8) {
        let cube = Cube {
         pos: (x, y, z)
        };
        self.cubes.push(cube);
    }
    pub fn get_vertices_indices(&self) -> (Vec<Vertex>, Vec<u16>) {
        let mut vertices = vec![];
        let mut indices = vec![];
        for (i, cube) in self.cubes.iter().enumerate() {
            let i = i as u16;
            vertices.extend(&[
                //top (0, 0, 1)
                Vertex::new([-1 + cube.pos.0, -1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, -1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, 1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, 1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                //bottom (0, 0, -1)
                Vertex::new([1 + cube.pos.0, 1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, 1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, -1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, -1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                //right (1, 0, 0)
                Vertex::new([1 + cube.pos.0, -1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, 1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, 1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, -1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                //left (-1, 0, 0)
                Vertex::new([-1 + cube.pos.0, 1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, -1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, -1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, 1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                //front (0, 1, 0)
                Vertex::new([-1 + cube.pos.0, 1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, 1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, 1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, 1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                //back (0, -1, 0)
                Vertex::new([0 + cube.pos.0, -1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, -1 + cube.pos.1, 1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([-1 + cube.pos.0, -1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
                Vertex::new([1 + cube.pos.0, -1 + cube.pos.1, -1 + cube.pos.2], CUBE_COLOR),
            ]);
            for face_index in 0..6 {
                let d = face_index + i;
                indices.extend(&[
                    4*d, 4*d + 1, 4*d + 2, 4*d + 2, 4*d + 3, 4*d
                ])
            }
            
        }
        (vertices, indices)
    }
}
