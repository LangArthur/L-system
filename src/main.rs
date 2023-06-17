extern crate kiss3d;
extern crate nalgebra as na;

mod lsystem;
mod renderer;

fn main() {
    let mut renderer = renderer::Renderer::new();
    renderer.render();

    // let sys = lsystem::DOLSystem::new();
}