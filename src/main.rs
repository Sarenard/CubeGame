#[macro_use]
extern crate glium;

use glium::{winit::{event::{DeviceEvent, ElementState, RawKeyEvent}, keyboard::{KeyCode, PhysicalKey}}, Surface};

mod models;
#[allow(unused)]
use models::cube as cube;
#[allow(unused)]
use models::teapot as teapot;

mod render;
use render::quaternion::Quaternion as Quaternion;
use render::vec3d::Vec3d as Vec3d;
use render::camera::Camera as Camera;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Cubegame")
        .build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &cube::VERTICES).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                        &cube::INDICES).unwrap();

    let vertex_shader_src = r#"
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

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                            None).unwrap();

    let mut camera = Camera::new();

    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
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

                    target.draw(&positions, &indices, &program, 
                        &uniform! { model: model, view: camera.render(), perspective: perspective },
                    &params).unwrap();
                    target.finish().unwrap();
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },
            // keyboard
            glium::winit::event::Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(RawKeyEvent { physical_key, state }) if state == ElementState::Pressed => {
                    println!("Physical key: {:?} State: {:?}", physical_key, state);
                    let direction = Vec3d::new(camera.direction.x, camera.direction.y, camera.direction.z).normalize();
                    let up = Vec3d::new(0.0, 1.0, 0.0);
                    let right = -direction.cross(up).normalize();
                    match physical_key {
                        PhysicalKey::Code(KeyCode::Escape) => {
                            panic!("Stop !");
                        }
                        // QWE
                        // ASD
                        // W - Avancer
                        PhysicalKey::Code(KeyCode::KeyW) => {
                            camera.deplace(direction);
                        }
                        // S - Reculer
                        PhysicalKey::Code(KeyCode::KeyS) => {
                            camera.deplace(-direction);
                        }
                        // A - Aller à gauche
                        PhysicalKey::Code(KeyCode::KeyA) => {
                            camera.deplace(-right);
                        }
                        // D - Aller à droite
                        PhysicalKey::Code(KeyCode::KeyD) => {
                            camera.deplace(right);
                        }
                        // E - Monter
                        PhysicalKey::Code(KeyCode::KeyE) => {
                            camera.deplace(up);
                        }
                        // Q - Descendre
                        PhysicalKey::Code(KeyCode::KeyQ) => {
                            camera.deplace(-up);
                        }
                        //  ^
                        // <_>
                        // rotation
                        PhysicalKey::Code(KeyCode::ArrowDown) => {
                            // Pitch down
                            let dir = Vec3d::new(camera.direction.x, camera.direction.y, camera.direction.z).normalize();
                            let right = Vec3d::new(dir.z, 0.0, -dir.x).normalize(); // axe horizontal
                            let angle = 0.1;
                            let rotation = Quaternion::from_rotation(right, angle);
                            camera.direction = (rotation * camera.direction).normalize();
                        }
                        PhysicalKey::Code(KeyCode::ArrowUp) => {
                            // Pitch up
                            let dir = Vec3d::new(camera.direction.x, camera.direction.y, camera.direction.z).normalize();
                            let right = Vec3d::new(dir.z, 0.0, -dir.x).normalize(); // axe horizontal
                            let angle = -0.1;
                            let rotation = Quaternion::from_rotation(right, angle);
                            camera.direction = (rotation * camera.direction).normalize();
                        }
                        PhysicalKey::Code(KeyCode::ArrowLeft) => {
                            // Yaw left
                            let axis = Vec3d::new(0.0, 1.0, 0.0); // axe vertical
                            let angle = -0.1;
                            let rotation = Quaternion::from_rotation(axis, angle);
                            camera.direction = (rotation * camera.direction).normalize();
                        }
                        PhysicalKey::Code(KeyCode::ArrowRight) => {
                            // Yaw right
                            let axis = Vec3d::new(0.0, 1.0, 0.0); // axe vertical
                            let angle = 0.1;
                            let rotation = Quaternion::from_rotation(axis, angle);
                            camera.direction = (rotation * camera.direction).normalize();
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
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();
}