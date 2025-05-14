#[macro_use]
extern crate glium;

mod models;
mod render;
mod world;

use world::world::World;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Cubegame")
        .build(&event_loop);

    let mut world = World::new(display, window);

    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        world.run(ev, window_target);
    }).unwrap();
}