#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            /// alternately use &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3]
            /// but rust gets a little confused and you need to change the lifetime of the layout to static,
            /// or use a const for the array, so for now I will define it explicitly.
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.35, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-0.35, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.35, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.35, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3
];
