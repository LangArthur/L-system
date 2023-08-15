use std::f32::consts::PI;

use na::{Point2, Point3, point};
use kiss3d::window::Window;
use kiss3d::light::Light;

use crate::lsystem::{QuadraticKochIsland, LSystem};

const COLOR: Point3<f32> = point![1.0f32, 0.0f32, 0.0f32];

pub enum Rotation {
    ClockWise,
    AntiClockWise,
}

impl Rotation {
    fn value(&self) -> f32 {
        match self {
            Self::ClockWise => 1.0,
            Self::AntiClockWise => -1.0,
        }
    }
}

pub struct Drawer {
    position: Point2<f32>,
    origin_position: Point2<f32>,
    angle: f32, // in radian
    origin_angle: f32,
}

impl Drawer {
    fn new(position: Point2<f32>, angle: f32) -> Self {
        Self {
            position,
            origin_position: position.clone(),
            angle,
            origin_angle: angle.clone(),
        }
    }

    fn reset(&mut self) {
        self.position = self.origin_position;
        self.angle = self.origin_angle;
    }
}

pub struct Renderer {
    win: Window,
    pen: Drawer,
    dist: f32,
    theta: f32, // in radian
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer = Self {
            win: Window::new("L-System"),
            pen: Drawer::new(point![0.0, 0.0], 0.5 * PI),
            dist: 10.0,
            theta: PI / 2.0,
        };
        renderer.win.set_light(Light::StickToCamera);
        // NOTE: origine position for drawing plants
        // let win_dim = renderer.win.size();
        // renderer.pen.position = point![0.0, -(win_dim.y as f32 / 2.0)];
        renderer
    }

    pub fn render(&mut self) {
        let mut system = QuadraticKochIsland::new();
        let sequence = system.get_step(2);
        while self.win.render() {
            self.draw(&sequence);
        }
    }

    fn draw(&mut self, sequence: &String) {
        // reset pen position
        self.pen.reset();
        for c in sequence.chars() {
            match c {
                'F' => self.draw_line(),
                'f' => self.move_pen(),
                '+' => self.rotate(Rotation::ClockWise),
                '-' => self.rotate(Rotation::AntiClockWise),
                _ => {},
            }
        }
    }

    fn draw_line(&mut self) {
        let start = self.pen.position;
        let end = point![
            self.pen.position.x + self.dist * (self.pen.angle as f32).cos(),
            self.pen.position.y + self.dist * (self.pen.angle as f32).sin()
        ];
        self.win.draw_planar_line(&start, &end, &COLOR);
        self.pen.position = end;
    }

    fn move_pen(&mut self) {
        self.pen.position = point![
            self.pen.position.x + self.dist * (self.pen.angle as f32).cos(),
            self.pen.position.y + self.dist * (self.pen.angle as f32).sin()
        ];
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.pen.angle = (self.pen.angle + self.theta * rotation.value()) % (PI * 2.0);
    }
}