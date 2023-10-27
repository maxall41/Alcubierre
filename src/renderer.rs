pub(crate) mod buffer;
pub(crate) mod sprite;
pub mod camera;

use hashbrown::HashMap;
use std::iter;
use std::num::NonZeroU32;
use cgmath::Rotation3;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Sampler, TextureView};
use wgpu::util::DeviceExt;
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::game_object::behaviours::EngineView;
use crate::renderer::camera::{Camera, CameraUniform};
use crate::ui::backend::wgpu::render_from_hyperfoil_ast;
use crate::ui::frontend::{HyperFoilAST, RGBColor};
use crate::MouseData;
use buffer::*;
use crate::renderer::sprite::{create_sprite_render_pipeline, SpriteInstance, SpriteVertex};
use crate::renderer::sprite::texture::Texture;

pub struct Render {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    sprite_render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    staging_belt: wgpu::util::StagingBelt,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera: camera::Camera,
    projection: camera::Projection,
    camera_bind_group: wgpu::BindGroup,
    glyph_brush: GlyphBrush<()>,
    size: PhysicalSize<u32>,
    font: FontArc,
    sprite_bind_group_layout: BindGroupLayout,
    sprite_vertex_buffer: Buffer,
    sprite_index_buffer: Buffer,
    sprite_instance_buffer: Buffer,
    sprite_num_indices: u32,
    diffuse_bind_group: BindGroup
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
        // warn!("Using adapter: {:?}", adapter.get_info());
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::TEXTURE_BINDING_ARRAY | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
                    limits: adapter.limits(),
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
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

        // Textures
        let texture_count = 1;
        let (sprite_render_pipeline,sprite_bind_group_layout) = create_sprite_render_pipeline(&device,&config,texture_count);


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

        const VERTICES: &[SpriteVertex] = &[
            SpriteVertex {
                position: [0.3381871, -0.3, 0.0],
                tex_coords: [1.0, 1.0],
            }, // A
            SpriteVertex {
                position: [0.9381871, -0.3, 0.0],
                tex_coords: [0.0, 1.0],
            }, // B
            SpriteVertex {
                position: [0.9381871, 0.3, 0.0],
                tex_coords: [0.0, 0.0],
            }, // C
            SpriteVertex {
                position: [0.3381871, 0.3, 0.0],
                tex_coords: [1.0, 0.0],
            }, // D
        ];

        const INDICES: &[u16] = &[0, 1, 2,0,2,3];

        let sprite_num_indices = INDICES.len() as u32;

        let sprite_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let sprite_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });


        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("./assets/monogram-default-font.ttf"))
                .unwrap();

        let glyph_brush = GlyphBrushBuilder::using_font(font.clone()).build(&device, config.format);

        let sprite_instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Sprite Instance Buffer"),
                contents: &[],
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

       // sprite_diffuse_bind_group


        let fake_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[],
            label: Some("fake_bind_group_layout"),
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &fake_bind_group_layout,
            entries: &[],
            label: Some("fake_diffuse_bind_group"),
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
            glyph_brush,
            size,
            font,
            sprite_render_pipeline,
            sprite_bind_group_layout,
            sprite_vertex_buffer,
            sprite_index_buffer,
            sprite_num_indices,
            sprite_instance_buffer,
            diffuse_bind_group
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera_uniform
                .update_view_proj(&self.camera, &self.projection);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );
        }
    }

    pub fn render_buffer(
        &mut self,
        mut buffer: QuadBufferBuilder,
        ast: &Option<HyperFoilAST>,
        data_map: &HashMap<String, String>,
        function_map: &HashMap<String, fn(&mut EngineView)>,
        engine_view: &mut EngineView,
        mouse_data: &MouseData,
        clear_color: &RGBColor,
        sprite_instances: Vec<SpriteInstance>,
        textures: Vec<&[u8]>
    ) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if ast.is_some() {
            render_from_hyperfoil_ast(
                ast.as_ref().unwrap(),
                &mut self.glyph_brush,
                self.size,
                &self.font,
                data_map,
                function_map,
                &mut buffer,
                engine_view,
                &self.projection,
                mouse_data,
            );
        }

        let (stg_vertex, stg_index, num_indices) = buffer.build(&self.device);

        stg_vertex.copy_to_buffer(&mut encoder, &self.vertex_buffer);
        stg_index.copy_to_buffer(&mut encoder, &self.index_buffer);

        match self.surface.get_current_texture() {
            Ok(frame) => {
                let view = frame.texture.create_view(&Default::default());

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            store: true,
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: clear_color.red as f64 / 255.0,
                                g: clear_color.green as f64 / 255.0,
                                b: clear_color.blue as f64 / 255.0,
                                a: 1.0,
                            }),
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                // Setup
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

                // Geometry
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.set_pipeline(&self.pipeline);
                render_pass.draw_indexed(0..num_indices, 0, 0..1);

                // Sprites

                let sprite_instance_data = sprite_instances.iter().map(SpriteInstance::to_raw).collect::<Vec<_>>();
                self.sprite_instance_buffer = self.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Sprite Instance Buffer"),
                        contents: bytemuck::cast_slice(&sprite_instance_data),
                        usage: wgpu::BufferUsages::VERTEX,
                    }
                );

                let mut loaded_textures: Vec<TextureView> = Vec::new();

                for texture in textures {
                    let diffuse_texture =
                        sprite::texture::Texture::from_bytes(&self.device, &self.queue, texture, "torch.png",false);
                    loaded_textures.push(diffuse_texture.view);
                }

                let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                });

                let x = loaded_textures.iter().map(| x | x).collect::<Vec<&TextureView>>();


                self.diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.sprite_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureViewArray(x.as_slice()),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: self.camera_buffer.as_entire_binding(),
                        }
                    ],
                    label: Some("diffuse_bind_group"),
                });

                render_pass.set_pipeline(&self.sprite_render_pipeline);
                render_pass.set_bind_group(2, &self.camera_bind_group, &[]);
                render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.sprite_vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, self.sprite_instance_buffer.slice(..));
                render_pass.set_index_buffer(self.sprite_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.sprite_num_indices, 0, 0..sprite_instances.len() as _);


                drop(render_pass);

                // Draw the text!
                self.glyph_brush
                    .draw_queued(
                        &self.device,
                        &mut self.staging_belt,
                        &mut encoder,
                        &view,
                        self.size.width,
                        self.size.height,
                    )
                    .expect("Draw queued");

                self.staging_belt.finish();
                self.queue.submit(iter::once(encoder.finish()));
                frame.present();
                self.staging_belt.recall();
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