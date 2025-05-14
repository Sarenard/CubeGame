use glium::winit::event::Event;
use glium::winit::event_loop::ActiveEventLoop;
use glium::winit::window::Window;
use glium::{glutin::surface::WindowSurface, Display};

use crate::models::cube;
use crate::models::object::Object;
use crate::render::camera::Camera as Camera;
use crate::glium::Surface;
use crate::render::quaternion::Quaternion;
use crate::render::vec3d::Vec3d;

use glium::winit::{event::{DeviceEvent, ElementState, RawKeyEvent}, keyboard::{KeyCode, PhysicalKey}};

pub struct World {
    pub camera: Camera,
    program: glium::Program,
    display: Display<WindowSurface>,
    window: Window,
    objects: Vec<Object>
}

const VERTEX_SHADER_SRC: &str = r#"
        #version 140

        in vec3 position;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

const FRAGMENT_SHADER_SRC: &str = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

impl World {
    pub fn new(display: Display<WindowSurface>, window: Window) -> World {
        let cube1 = cube::new([0., 0., 0.]);
        let cube2 = cube::new([100., 0., 0.]);
        
        World {
            camera: Camera::new(),
            program: glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC,
                None).unwrap(),
            display: display,
            window: window,
            objects: vec![cube1, cube2],
        }
    }

    pub fn render(&self) {
        let mut target = self.display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
    
            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;
    
            let f = 1.0 / (fov / 2.0).tan();
    
            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        for elt in &self.objects {
            let positions = glium::VertexBuffer::new(&self.display, &elt.vertices).unwrap();
            let indices = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TrianglesList,
                                                &elt.indices).unwrap();
            
            target.draw(&positions, &indices, &self.program, 
                &uniform! { model: model, view: self.camera.render(), perspective: perspective },
            &params).unwrap();
        }

        target.finish().unwrap();

    }

    pub fn run(&mut self, ev: Event<()>, window_target: &ActiveEventLoop) {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                glium::winit::event::WindowEvent::RedrawRequested => {
                    self.render();
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    self.display.resize(window_size.into());
                },
                _ => (),
            },
            // keyboard
            glium::winit::event::Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(RawKeyEvent { physical_key, state }) if state == ElementState::Pressed => {
                    println!("Physical key: {:?} State: {:?}", physical_key, state);
                    let direction = Vec3d::new(self.camera.direction.x, self.camera.direction.y, self.camera.direction.z).normalize();
                    let up = Vec3d::new(0.0, 1.0, 0.0);
                    let right = -direction.cross(up).normalize();
                    match physical_key {
                        PhysicalKey::Code(KeyCode::Escape) => {
                            window_target.exit();
                        }
                        // QWE
                        // ASD
                        // W - Avancer
                        PhysicalKey::Code(KeyCode::KeyW) => {
                            self.camera.deplace(direction);
                        }
                        // S - Reculer
                        PhysicalKey::Code(KeyCode::KeyS) => {
                            self.camera.deplace(-direction);
                        }
                        // A - Aller à gauche
                        PhysicalKey::Code(KeyCode::KeyA) => {
                            self.camera.deplace(-right);
                        }
                        // D - Aller à droite
                        PhysicalKey::Code(KeyCode::KeyD) => {
                            self.camera.deplace(right);
                        }
                        // E - Monter
                        PhysicalKey::Code(KeyCode::KeyE) => {
                            self.camera.deplace(up);
                        }
                        // Q - Descendre
                        PhysicalKey::Code(KeyCode::KeyQ) => {
                            self.camera.deplace(-up);
                        }
                        //  ^
                        // <_>
                        // rotation
                        PhysicalKey::Code(KeyCode::ArrowDown) => {
                            // Pitch down
                            let dir = Vec3d::new(self.camera.direction.x, self.camera.direction.y, self.camera.direction.z).normalize();
                            let right = Vec3d::new(dir.z, 0.0, -dir.x).normalize(); // axe horizontal
                            let angle = 0.1;
                            let rotation = Quaternion::from_rotation(right, angle);
                            self.camera.direction = (rotation * self.camera.direction).normalize();
                        }
                        PhysicalKey::Code(KeyCode::ArrowUp) => {
                            // Pitch up
                            let dir = Vec3d::new(self.camera.direction.x, self.camera.direction.y, self.camera.direction.z).normalize();
                            let right = Vec3d::new(dir.z, 0.0, -dir.x).normalize(); // axe horizontal
                            let angle = -0.1;
                            let rotation = Quaternion::from_rotation(right, angle);
                            self.camera.direction = (rotation * self.camera.direction).normalize();
                        }
                        PhysicalKey::Code(KeyCode::ArrowLeft) => {
                            // Yaw left
                            let axis = Vec3d::new(0.0, 1.0, 0.0); // axe vertical
                            let angle = -0.1;
                            let rotation = Quaternion::from_rotation(axis, angle);
                            self.camera.direction = (rotation * self.camera.direction).normalize();
                        }
                        PhysicalKey::Code(KeyCode::ArrowRight) => {
                            // Yaw right
                            let axis = Vec3d::new(0.0, 1.0, 0.0); // axe vertical
                            let angle = 0.1;
                            let rotation = Quaternion::from_rotation(axis, angle);
                            self.camera.direction = (rotation * self.camera.direction).normalize();
                        }
                        // other
                        PhysicalKey::Code(unknown) => {
                            println!("Unknown key: {:?}", unknown);
                        }
                        PhysicalKey::Unidentified(_) => {
                            println!("Unidentified key");
                        }
                    }
                },
                _ => ()
            },
            glium::winit::event::Event::AboutToWait => {
                self.window.request_redraw();
            },
            _ => (),
        }
    }
}