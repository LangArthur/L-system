extern crate kiss3d;
extern crate nalgebra as na;

mod lsystem;
mod renderer;

fn main() {
    // let mut renderer = renderer::Renderer::new();
    // renderer.render();

    let mut sys = lsystem::DOLSystem::new();
    for i in 0..5 {
        println!("{}", sys.get_step(i));
    }
}