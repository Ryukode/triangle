mod buffers;
mod mesh;
mod model;
mod matrix;
mod camera;
mod vector;
mod transform;
mod color;
mod shader;

use std::env;
use std::f32::consts::PI;
use std::sync::Arc;
use wgpu::{include_wgsl, BindGroupLayout, Face, FrontFace, PrimitiveTopology, RenderPipeline, ShaderModuleDescriptor, ShaderSource};
use wgpu::Face::Back;
use wgpu::PolygonMode::Fill;
use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use crate::model::Model;
use crate::camera::Camera;
use crate::color::Color;
use crate::shader::{AnyShader, BaseShader, PhongShader, FlatShader};
use crate::vector::Vector3;

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: RenderPipeline,
    window: Arc<Window>,
    uniform_bind_group_layout: BindGroupLayout,
}

impl<'a> State<'a> {
    async fn new(event_loop: &ActiveEventLoop) -> Self {

        let instance = wgpu::Instance::default();

        let window_attributes = Window::default_attributes().with_title("resumed");

        let window : Arc<Window> = Arc::new(event_loop.create_window(window_attributes).unwrap());

        window.set_title("triangle");
        let surface = instance.create_surface(window.clone()).expect("Failed to create surface!");

        let size = window.inner_size();
        

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap();
        
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();        

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 1,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let uniform_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
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
                label: Some("uniform_bgl"),
            }
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(include_wgsl!("shaders/phong.wgsl"));

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[buffers::VertexBuffer::LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Back),
                unclipped_depth: false,
                polygon_mode: Fill,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            window,
            uniform_bind_group_layout
        } 
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self, model: &Model, mut uniforms: Vec<f32>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&*model.get_vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&*model.get_indices()),
                usage: wgpu::BufferUsages::INDEX,
            });

            let uniform_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(uniforms.as_mut_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let uniform_bind_group = self.device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &self.uniform_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }],
                    label: Some("uniform_bg"),
                }
            );

            render_pass.set_bind_group(0, &uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw_indexed(0..model.get_indices().len() as u32, 0, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}

struct App<'a> {
    state: Option<State<'a>>,
    models: Vec<Model>,
    camera: Camera,
    key_event: Option<KeyEvent>,
}

impl App<'_> {
    fn handle_key_event(&mut self) {
        match &self.key_event {
            Some(event) => {
                match (event.physical_key, event.state) {
                    (PhysicalKey::Code(KeyCode::KeyD), ElementState::Pressed) => {
                        let current_pos = self.camera.transform.get_position();
                        self.camera.transform.set_position(current_pos + Vector3::new(-0.1,0.,0.));
                    },
                    (PhysicalKey::Code(KeyCode::KeyA), ElementState::Pressed) => {
                        let current_pos = self.camera.transform.get_position();
                        self.camera.transform.set_position(current_pos + Vector3::new(0.1,0.,0.));
                    },
                    (PhysicalKey::Code(KeyCode::KeyW), ElementState::Pressed) => {
                        let current_pos = self.camera.transform.get_position();
                        self.camera.transform.set_position(current_pos + Vector3::new(0.,0.,0.1));
                    },
                    (PhysicalKey::Code(KeyCode::KeyS), ElementState::Pressed) => {
                        let current_pos = self.camera.transform.get_position();
                        self.camera.transform.set_position(current_pos + Vector3::new(0.,0.,-0.1));
                    },
                    (PhysicalKey::Code(KeyCode::KeyQ), ElementState::Pressed) => {
                        let current_pos = self.camera.transform.get_position();
                        self.camera.transform.set_position(current_pos + Vector3::new(0.,-0.1,0.));
                    },
                    (PhysicalKey::Code(KeyCode::KeyE), ElementState::Pressed) => {
                        let current_pos = self.camera.transform.get_position();
                        self.camera.transform.set_position(current_pos + Vector3::new(0.,0.1,0.));
                    },
                    _ => {}
                }
            }
            None => { }
        }
        self.key_event = None;
    }

    fn load_model() -> Model {
        let mut model: Model = Model::default();
        let args: Vec<String> = env::args().collect();
        if args.len() <= 1{
            let _ = model.load_obj("Assets/pawn.obj");
        }
        else {
            let _ = model.load_obj(&args[1]);
        }
        println!("{}", model);
        model
    }

    fn start(&mut self) {
        self.models[0].transform.set_position(Vector3::new(0., 0., 0.));
        self.camera.transform.set_position(Vector3::new(0., 0.,-5.));
    }

    fn update(&mut self) {
        self.handle_key_event();
    }

    fn draw(&mut self) {
        let s: &mut State = self.state.as_mut().unwrap();

        let flat = FlatShader::default();

        let mut phong = PhongShader::default();
        phong.set_ambient(Color::new(1., 0., 0., 1.));
        phong.set_diffuse(Color::new(1., 0., 0., 1.));
        phong.set_specular(Color::new(1., 1., 1., 1.));
        phong.set_eye_pos(self.camera.transform.get_position());
        phong.set_light_dir(Vector3::new(-1., -1., 1.));

        for i in 0..self.models.len(){
            s.render(&self.models[i], phong.as_vec(&self.models[i], &self.camera)).expect("Render failed")
        }

        s.window.request_redraw();
    }
}

const SCREEN_WIDTH: i32 = 600;
const SCREEN_HEIGHT: i32 = 420;
impl<'a> Default for App<'a> {
    fn default() -> Self {
        let fov = PI * 65. / 180.;
        let aspect = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
        let near = 0.001;
        let far = 1000.;

        let mut models: Vec<Model> = Vec::new();
        models.push(Self::load_model());

        let mut app: App = App {
            state: None,
            models,
            camera: Camera::new(fov, aspect, near, far),
            key_event: None
        };

        app.start();

        app
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {   
        self.state = Some(pollster::block_on(State::new(event_loop)));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let _ = self.update();
                let _ = self.draw();
            },
            WindowEvent::KeyboardInput {device_id, event, is_synthetic} => {
                self.key_event = Some(event);
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
