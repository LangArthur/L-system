extern crate kiss3d;
extern crate nalgebra as na;

mod lsystem;
mod renderer;
mod config;
mod math;
mod error;

fn main() {
    let mut renderer = renderer::Renderer::new();
    renderer.render();
}