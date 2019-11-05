extern crate camera_controllers;
extern crate piston_window;
extern crate vecmath;
#[macro_use]
extern crate gfx;
extern crate shader_version;

use crate::util::Vertex;
use cubes::Scene;

mod sim;
mod util;
mod cubes;

//----------------------------------------
// Cube associated data

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "Target0",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

//----------------------------------------

fn main() {
    use camera_controllers::{model_view_projection, Camera, CameraPerspective};
    use gfx::traits::*;
    use piston_window::*;
    use shader_version::glsl::GLSL;
    use shader_version::Shaders;

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("piston: cube", [640, 480])
        .exit_on_esc(true)
        .samples(4)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    let ref mut factory = window.factory.clone();

    let mut scene = Scene::new();

    scene.add_cube(1.0, 1.0);

    let (vertex_data, index_data) = scene.get_vertices_indices();
    let index_data_slice = &index_data[..];

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data_slice);

    let glsl = opengl.to_glsl();
    let pso = factory
        .create_pipeline_simple(
            Shaders::new()
                .set(GLSL::V1_50, include_str!("../assets/cube_150.glslv"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            Shaders::new()
                .set(GLSL::V1_50, include_str!("../assets/cube_150.glslf"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            pipe::new(),
        )
        .unwrap();

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 90.0,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32),
        }
        .projection()
    };

    let model = vecmath::mat4_id();
    let mut projection = get_projection(&window);
    let camera = Camera::new([2.0, 4.0, 7.0]);

    let mut data = pipe::Data {
        vbuf: vbuf.clone(),
        u_model_view_proj: [[0.0; 4]; 4],
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
    };

    while let Some(e) = window.next() {
        window.draw_3d(&e, |window| {
            window
                .encoder
                .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj = model_view_projection(model, camera.orthogonal(), projection);
            window.encoder.draw(&slice, &pso, &data);
        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
            data.out_color = window.output_color.clone();
            data.out_depth = window.output_stencil.clone();
        }
    }
}
