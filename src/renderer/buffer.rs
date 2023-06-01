use crate::game_object::graphics::RectData;
use crate::renderer::circle::CircleVertex;
use crate::ui::frontend::RGBColor;
use bytemuck::{Pod, Zeroable};
use cgmath::num_traits::Pow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub const U32_SIZE: wgpu::BufferAddress = std::mem::size_of::<u32>() as wgpu::BufferAddress;
pub const F32_SIZE: wgpu::BufferAddress = std::mem::size_of::<f32>() as wgpu::BufferAddress;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    #[allow(dead_code)]
    pub(crate) position: [f32; 2],
    #[allow(dead_code)]
    pub(crate) color: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Color {
    #[allow(dead_code)]
    pub(crate) r: f32,
    #[allow(dead_code)]
    pub(crate) g: f32,
    #[allow(dead_code)]
    pub(crate) b: f32,
    #[allow(dead_code)]
    pub(crate) a: f32,
}

pub fn size_of_slice<T: Sized>(slice: &[T]) -> usize {
    std::mem::size_of::<T>() * slice.len()
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Color {}
unsafe impl bytemuck::Zeroable for Color {}

impl Vertex {
    pub const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    pub const DESC: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: Self::SIZE,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x3
        ],
    };
}

pub struct QuadBufferBuilder {
    vertex_data: Vec<Vertex>,
    circle_vertex_data: Vec<CircleVertex>,
    index_data: Vec<u32>,
    circle_index_data: Vec<u32>,
    current_quad: u32,
    current_circle_quad: u32,
}

impl QuadBufferBuilder {
    pub fn new() -> Self {
        Self {
            vertex_data: vec![],
            index_data: vec![],
            circle_vertex_data: vec![],
            circle_index_data: vec![],
            current_quad: 0,
            current_circle_quad: 0,
        }
    }

    pub fn push_square(&mut self, x: f32, y: f32, width: f32, height: f32, color: &RGBColor) {
        self.push_quad(
            x - width * 0.5,
            y - height * 0.5,
            x + width * 0.5,
            y + height * 0.5,
            color,
        );
    }

    pub fn push_quad(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32, color: &RGBColor) {
        let red = ((color.red as f32 / 255.0 + 0.055) / 1.055).pow(2.4);
        let green = ((color.green as f32 / 255.0 + 0.055) / 1.055).pow(2.4);
        let blue = ((color.blue as f32 / 255.0 + 0.055) / 1.055).pow(2.4);

        let vertices = &[
            Vertex {
                position: [min_x, min_y],
                color: [red, green, blue],
            },
            Vertex {
                position: [max_x, min_y],
                color: [red, green, blue],
            },
            Vertex {
                position: [max_x, max_y],
                color: [red, green, blue],
            },
            Vertex {
                position: [min_x, max_y],
                color: [red, green, blue],
            },
        ];
        self.vertex_data.extend(vertices);

        self.index_data.extend(&[
            self.current_quad * 4 + 0,
            self.current_quad * 4 + 1,
            self.current_quad * 4 + 2,
            self.current_quad * 4 + 0,
            self.current_quad * 4 + 2,
            self.current_quad * 4 + 3,
        ]);
        self.current_quad += 1;
    }

    pub fn push_circle_quad(
        &mut self,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
        color: &RGBColor,
        radius: f32,
    ) {
        let red = ((color.red as f32 / 255.0 + 0.055) / 1.055).pow(2.4);
        let green = ((color.green as f32 / 255.0 + 0.055) / 1.055).pow(2.4);
        let blue = ((color.blue as f32 / 255.0 + 0.055) / 1.055).pow(2.4);

        let vertices = &[
            CircleVertex {
                position: [min_x, min_y],
                color: [red, green, blue],
                radius,
            },
            CircleVertex {
                position: [max_x, min_y],
                color: [red, green, blue],
                radius,
            },
            CircleVertex {
                position: [max_x, max_y],
                color: [red, green, blue],
                radius,
            },
            CircleVertex {
                position: [min_x, max_y],
                color: [red, green, blue],
                radius,
            },
        ];

        // let vertices: &[CircleVertex] = &[
        //     CircleVertex {
        //         position: [-0.49157882, -2.3097796],
        //         color: [1.0, 1.0, 1.0],
        //         radius: 0.002,
        //     },
        //     CircleVertex {
        //         position: [0.5084212, -2.3097796],
        //         color: [1.0, 1.0, 1.0],
        //         radius: 0.002,
        //     },
        //     CircleVertex {
        //         position: [0.5084212, -1.3097795],
        //         color: [1.0, 1.0, 1.0],
        //         radius: 0.002,
        //     },
        //     CircleVertex {
        //         position: [-0.49157882, -1.3097795],
        //         color: [1.0, 1.0, 1.0],
        //         radius: 0.002,
        //     },
        // ];

        self.circle_vertex_data.extend(vertices);

        self.circle_index_data.extend(&[
            self.current_circle_quad * 4 + 0,
            self.current_circle_quad * 4 + 1,
            self.current_circle_quad * 4 + 2,
            self.current_circle_quad * 4 + 0,
            self.current_circle_quad * 4 + 2,
            self.current_circle_quad * 4 + 3,
        ]);
        self.current_circle_quad += 1;
    }

    pub fn push_circle(&mut self, pos_x: f32, pos_y: f32, radius: f32, color: &RGBColor) {
        let width = 2.0;
        let height = 2.0;

        self.push_circle_quad(
            pos_x - width * 0.5,
            pos_y - height * 0.5,
            pos_x + width * 0.5,
            pos_y + height * 0.5,
            &color,
            radius,
        );
    }

    pub fn build(
        self,
        device: &wgpu::Device,
    ) -> (
        StagingBuffer,
        StagingBuffer,
        StagingBuffer,
        StagingBuffer,
        u32,
        u32,
    ) {
        (
            StagingBuffer::new(device, &self.vertex_data, false),
            StagingBuffer::new(device, &self.index_data, true),
            StagingBuffer::new(device, &self.circle_vertex_data, false),
            StagingBuffer::new(device, &self.circle_index_data, true),
            self.index_data.len() as u32,
            self.circle_index_data.len() as u32,
        )
    }
}

pub struct StagingBuffer {
    buffer: wgpu::Buffer,
    size: wgpu::BufferAddress,
}

impl StagingBuffer {
    pub fn new<T: bytemuck::Pod + Sized>(
        device: &wgpu::Device,
        data: &[T],
        is_index_buffer: bool,
    ) -> StagingBuffer {
        StagingBuffer {
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::COPY_SRC
                    | if is_index_buffer {
                        wgpu::BufferUsages::INDEX
                    } else {
                        wgpu::BufferUsages::empty()
                    },
                label: Some("Staging Buffer"),
            }),
            size: size_of_slice(data) as wgpu::BufferAddress,
        }
    }

    pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder, other: &wgpu::Buffer) {
        encoder.copy_buffer_to_buffer(&self.buffer, 0, other, 0, self.size)
    }
}
