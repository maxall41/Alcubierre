pub(crate) mod buffer;
pub mod camera;
pub mod circle;

use std::iter;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::renderer::camera::{Camera, CameraUniform};
use crate::renderer::circle::CircleVertex;
use buffer::*;

pub struct Render {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    circle_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    circle_index_buffer: wgpu::Buffer,
    staging_belt: wgpu::util::StagingBelt,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    circle_buffer: wgpu::Buffer,
    camera: camera::Camera,         // UPDATED!
    projection: camera::Projection, // NEW!
    camera_bind_group: wgpu::BindGroup,
}

impl Render {
    pub fn width(&self) -> f32 {
        self.config.width as f32
    }

    #[allow(dead_code)]
    pub fn height(&self) -> f32 {
        self.config.height as f32
    }

    pub async fn new(window: &Window, size: PhysicalSize<u32>) -> Self {
        // log::warn!("size: {:?}", size);
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        // Camera system

        let mut camera_uniform = CameraUniform::new();

        let camera = Camera {
            eye: (0.0, 0.0, 6.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
        };

        let projection =
            camera::Projection::new(config.width, config.height, cgmath::Deg(90.0), 0.1, 100.0);

        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        //

        surface.configure(&device, &config);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
            label: Some("Pipeline Layout"),
        });

        let pipeline = create_render_pipeline(
            &device,
            &pipeline_layout,
            config.format,
            &[Vertex::DESC],
            wgpu::include_wgsl!("./renderer/shaders/quad.wgsl"),
        );

        let circle_pipeline = create_render_pipeline(
            &device,
            &pipeline_layout,
            config.format,
            &[CircleVertex::DESC],
            wgpu::include_wgsl!("./renderer/shaders/circle.wgsl"),
        );

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: Vertex::SIZE * 4 * 2000,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Index Buffer"),
            size: U32_SIZE * 6 * 2000,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let circle_index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Circle Index Buffer"),
            size: U32_SIZE * 6 * 2000,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let circle_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Circle Vertex Buffer"),
            size: CircleVertex::SIZE * 4 * 2000,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            surface,
            adapter,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            index_buffer,
            staging_belt,
            camera_bind_group: bind_group,
            camera_buffer,
            camera_uniform,
            projection,
            camera,
            circle_buffer: circle_buf,
            circle_pipeline,
            circle_index_buffer,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.projection.resize(size.width, size.height);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
    }

    pub fn render_buffer(&mut self, buffer: QuadBufferBuilder) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let (stg_vertex, stg_index, circle_vert, circle_index, num_indices, circle_num_indices) = buffer.build(&self.device);

        stg_vertex.copy_to_buffer(&mut encoder, &self.vertex_buffer);
        stg_index.copy_to_buffer(&mut encoder, &self.index_buffer);
        circle_vert.copy_to_buffer(&mut encoder, &self.circle_buffer);
        circle_index.copy_to_buffer(&mut encoder,&self.circle_index_buffer);

        match self.surface.get_current_texture() {
            Ok(frame) => {
                let view = frame.texture.create_view(&Default::default());
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations::default(),
                    })],
                    depth_stencil_attachment: None,
                });

                // Setup
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

                // Squares
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.set_pipeline(&self.pipeline);
                render_pass.draw_indexed(0..num_indices, 0, 0..1);

                // Circles
                render_pass.set_vertex_buffer(0, self.circle_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.circle_index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.set_pipeline(&self.circle_pipeline);
                render_pass.draw_indexed(0..circle_num_indices, 0, 0..1);

                drop(render_pass);

                self.staging_belt.finish();
                self.queue.submit(iter::once(encoder.finish()));
                frame.present();
            }
            Err(wgpu::SurfaceError::Outdated) => {
                println!("Outdated surface texture");
                self.surface.configure(&self.device, &self.config);
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    src: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(src);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
            // or Features::POLYGON_MODE_POINT
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    })
}
