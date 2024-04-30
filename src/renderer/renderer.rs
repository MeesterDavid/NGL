use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::vertex_array::VertexArray;
use crate::set_attribute;
use image::ImageError;
use std::ptr;
use thiserror::Error;
use ultraviolet::Mat4;
use std::time::Instant;

/// Takes a string literal and concatenates a null byte onto the end.
#[macro_export]
macro_rules! null_str {
  ($lit:literal) => {{
    // "type check" the input
    const _: &str = $lit;
    concat!($lit, "\0")
  }};
}



pub fn get_error(message: &str) {
    let error: u32;
    unsafe {
        error = gl::GetError();
    }

    println!("{}\t{}", message,
    match error {
        gl::INVALID_ENUM => "invalid enum",
        gl::INVALID_VALUE => "invalid value",
        gl::INVALID_OPERATION => "invalid operation",
        gl::INVALID_FRAMEBUFFER_OPERATION => "invalid framebuffer operation",
        gl::OUT_OF_MEMORY => "out of memory",
        gl::NO_ERROR => "no error",
        _ => "?"

    });
    
    if error != gl::NO_ERROR {
        std::process::exit(1);
    }
}

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
uniform mat4 transform;
uniform mat4 y_transform;

in vec3 position;
in vec3 color;
out vec3 vertexColor;

void main() {
    gl_Position = (transform + y_transform) * vec4(position, 1.0);
    vertexColor = color;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
in vec3 vertexColor;
out vec4 fragColor;

void main() {
    fragColor = vec4(vertexColor, 1.0);
}
"#;

type Pos = [f32; 3];
type Color = [f32; 3];

#[repr(C, packed)]
struct Vertex(Pos, Color);
type TriIndex = [u32; 3];


#[rustfmt::skip]
const VERTICES: [Vertex; 4] = [
    Vertex([-0.0,  1.0,  0.0],  [0.0, 1.0, 0.0]),
    Vertex([-1.0,  0.0, -1.0],  [1.0, 1.0, 0.0]),
    Vertex([ 1.0,  0.0,  0.0],  [1.0, 0.0, 1.0]),
    Vertex([ 0.0,  0.0,  1.0],  [1.0, 1.0, 0.0]),
];


#[rustfmt::skip]
const INDICES: [TriIndex; 4] = [
    [0, 1, 2],
    [0, 3, 2],
    [0, 1, 3],
    [1, 2, 3],
];

// #[rustfmt::skip]
// const CUBE: [Vertex; 6] = [
//     Vertex([ 0.5,  0.5, -0.5],  [0.0, 1.0, 0.0]),
//     Vertex([ 0.5,  0.5, -0.5],  [1.0, 1.0, 0.0]),
//     Vertex([ 0.5,  0.5, -0.5],  [1.0, 0.0, 1.0]),
//     Vertex([-0.5,  0.5,  0.5],  [1.0, 1.0, 0.0]),
//     Vertex([-0.5,  0.5,  0.5],  [1.0, 1.0, 0.0]),
//     Vertex([-0.5,  0.5,  0.5],  [1.0, 1.0, 0.0]),
// ];

// #[rustfmt::skip]
// const CUBE_INDICES: [TriIndex; 2] = [
//     [0, 1, 2],
//     [2, 3, 0],
//     []
// ];


#[derive(Debug, Error)]
pub enum RendererInitError {
    #[error{"{0}"}]
    ImageError(#[from] ImageError),
    #[error{"{0}"}]
    ShaderError(#[from] ShaderError),
}

pub struct Renderer {
    program: ShaderProgram,
    _vertex_buffer: Buffer,
    _index_buffer: Buffer,
    vertex_array: VertexArray,
    start_time: Instant,
    angle: f32,
    // texture0: Texture,
    // texture1: Texture,
}

impl Renderer {
    pub fn new() -> Result<Self, RendererInitError> {
        unsafe {
            let start_time = Instant::now();
            let angle = 1.0;

            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&INDICES, gl::STATIC_DRAW);

            let pos_attrib = program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, Vertex::0);
            let color_attrib = program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, Vertex::1);

            // let texture0 = Texture::new();
            // texture0.set_wrapping(gl::REPEAT);
            // texture0.set_filtering(gl::LINEAR);
            // texture0.load(&Path::new("assets/logo.png"))?;
            // program.set_int_uniform("texture0", 0)?;

            // let texture1 = Texture::new();
            // texture1.set_wrapping(gl::REPEAT);
            // texture1.set_filtering(gl::LINEAR);
            // texture1.load(&Path::new("assets/rust.jpg"))?;
            // program.set_int_uniform("texture1", 1)?;

            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // gl::Enable(gl::BLEND);

            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                _index_buffer: index_buffer,
                vertex_array,
                start_time,
                angle,
                // texture0,
                // texture1,
            })
        }
    }

    pub fn draw(& mut self) {
        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            get_error("na clear color");

            gl::Clear(gl::COLOR_BUFFER_BIT);
            get_error("na clear");

            // self.texture0.activate(gl::TEXTURE0);
            // self.texture1.activate(gl::TEXTURE1);
                // update the "world state".
            let time =  self.start_time.elapsed().as_secs_f32();

            println!("time: {}", time);

            let transform = Mat4::from_rotation_z(time);
            let y_transform = Mat4::from_rotation_y(time);

            self.program.apply();
            let transform_name: *const i8 = (null_str!("transform")).as_ptr().cast();
            let y_transform_name: *const i8 = (null_str!("y_transform")).as_ptr().cast();

            get_error("na transform name");

            let transform_loc = gl::GetUniformLocation(self.program.id, transform_name);
            let y_transform_loc = gl::GetUniformLocation(self.program.id, y_transform_name);
            // // //let transform: *const f32 = []

            let transform_ptr: *const f32 = transform.as_ptr();
            let y_transform_ptr: *const f32 = y_transform.as_ptr();

            // let test: [gl::types::GLfloat; 16] = [
            //     -0.4, 0.9, 0.0, 0.0, 
            //     -0.9, -0.4, 0.0, 0.0, 
            //     0.0, 0.0, 1.0, 0.0, 
            //     0.0, 0.0, 0.0, 1.0
            // ];

            get_error("na get uniform location");
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform_ptr);
            gl::UniformMatrix4fv(y_transform_loc, 1, gl::FALSE, y_transform_ptr);

            get_error("na uniform matrix");

            // self.program.apply();
            self.vertex_array.bind();

            get_error("na vertex bind");

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            get_error("na draw elements");

        }
    }
}