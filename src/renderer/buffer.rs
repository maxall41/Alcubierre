use crate::ui::frontend::RGBColor;

use cgmath::num_traits::Pow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub const U32_SIZE: wgpu::BufferAddress = std::mem::size_of::<u32>() as wgpu::BufferAddress;

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
    index_data: Vec<u32>,
    current_vert: u32,
}

impl QuadBufferBuilder {
    pub fn new() -> Self {
        Self {
            vertex_data: vec![],
            index_data: vec![],
            current_vert: 0,
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
            self.current_vert + 0,
            self.current_vert + 1,
            self.current_vert + 2,
            self.current_vert + 0,
            self.current_vert + 2,
            self.current_vert + 3,
        ]);
        self.current_vert += 4;
    }

    pub fn push_circle(&mut self, pos_x: f32, pos_y: f32, mut radius: f32, color: &RGBColor,sides: u8) {
        // Convert color to sRGB
        let red = ((color.red as f32 / 255.0 + 0.055) / 1.055).pow(2.4);
        let green = ((color.green as f32 / 255.0 + 0.055) / 1.055).pow(2.4);
        let blue = ((color.blue as f32 / 255.0 + 0.055) / 1.055).pow(2.4);

        let mut rot : f32 = 0.0;

        self.vertex_data.push(Vertex {
            position: [pos_x,pos_y],
            color: [red,green,blue],
        });
        self.current_vert += 1;

        for i in 0..=sides {
            let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
            let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();
            self.vertex_data.push(Vertex {
                position: [pos_x + radius * rx,pos_y + radius * ry],
                color: [red,green,blue],
            });
            self.current_vert += 1;

            if i != sides {
                self.index_data.extend_from_slice(&[0, i as u32 + 1, i as u32 + 2]);
            }
        }
    }

    pub fn build(
        self,
        device: &wgpu::Device,
    ) -> (
        StagingBuffer,
        StagingBuffer,
        u32,
    ) {
        (
            StagingBuffer::new(device, &self.vertex_data, false),
            StagingBuffer::new(device, &self.index_data, true),
            self.index_data.len() as u32,
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
