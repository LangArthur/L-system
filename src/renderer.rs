use na::{Vector3, UnitQuaternion, Point2, point};
use kiss3d::window::Window;
use kiss3d::light::Light;

pub struct Renderer {
    win: Window,
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer = Self {
            win: Window::new("L-System"),
        };
        renderer.win.set_light(Light::StickToCamera);
        renderer
    }

    pub fn render(self: &mut Self) {
        while self.win.render() {
            self.win.draw_planar_line(&point![100.0f32, 0.0f32], &point![100.0f32, 1000.0f32], &point![1.0f32, 0.0f32, 0.0f32]);
            // c.prepend_to_local_rotation(&rot);
        }    
    }
}