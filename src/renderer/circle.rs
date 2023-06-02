#[derive(Copy, Clone, Debug)]
pub struct CircleVertex {
    #[allow(dead_code)]
    pub(crate) position: [f32; 2],
    #[allow(dead_code)]
    pub(crate) color: [f32; 3],
    #[allow(dead_code)]
    pub(crate) radius: f32,
    #[allow(dead_code)]
    pub(crate) model_matrix: [f32; 2]
}

impl CircleVertex {
    pub const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    pub const DESC: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: Self::SIZE,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x3,
            2 => Float32,
            3 => Float32x2
        ],
    };
}

unsafe impl bytemuck::Pod for CircleVertex {}
unsafe impl bytemuck::Zeroable for CircleVertex {}
