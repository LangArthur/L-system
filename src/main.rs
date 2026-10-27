extern crate kiss3d;

mod config;
mod error;
mod lsystem;
mod math;
mod renderer;
mod utility;

#[kiss3d::main]
async fn main() {
    let mut renderer = renderer::Renderer::new().await;
    renderer.render().await;
}
