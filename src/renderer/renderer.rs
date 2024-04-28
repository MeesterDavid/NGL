use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::texture::Texture;
use crate::renderer::vertex_array::VertexArray;
use crate::set_attribute;
use image::ImageError;
use std::path::Path;
use std::ptr;
use thiserror::Error;

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec3 position;
in vec3 color;
out vec3 vertexColor;

void main() {
    gl_Position = vec4(position, 1.0);
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
    Vertex([ 0.5,  0.5,  0.0],  [0.0, 1.0, 0.0]),
    Vertex([ 0.5, -0.5,  0.0],  [1.0, 1.0, 0.0]),
    Vertex([ 0.5,  0.5,  0.0],  [1.0, 0.0, 1.0]),
    Vertex([-0.5,  0.5,  0.0],  [1.0, 1.0, 0.0]),
];

#[rustfmt::skip]
const INDICES: [TriIndex; 2] = [
    [0, 1, 2],
    [2, 3, 0]
];


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
    // texture0: Texture,
    // texture1: Texture,
}

impl Renderer {
    pub fn new() -> Result<Self, RendererInitError> {
        unsafe {
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
                // texture0,
                // texture1,
            })
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // self.texture0.activate(gl::TEXTURE0);
            // self.texture1.activate(gl::TEXTURE1);
            
            self.program.apply();
            self.vertex_array.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}