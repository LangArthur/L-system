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
    states: Vec<DrawerState>,
}

impl Drawer {
    fn new(position: Point2<f32>, angle: f32) -> Self {
        Self {
            position,
            origin_position: position.clone(),
            angle,
            origin_angle: angle.clone(),
            states: vec![], // this vector serve as a stack
        }
    }

    fn reset(&mut self) {
        self.position = self.origin_position;
        self.angle = self.origin_angle;
    }

    fn save_state(&mut self) {
        self.states.push(DrawerState {
            position: self.position,
            angle: self.angle,
        })
    }

    fn load_last_state(&mut self) {
        if let Some(state) = self.states.pop() {
            self.position = state.position;
            self.angle = state.angle;
        }
    }
}

pub struct DrawerState {
    position: Point2<f32>,
    angle: f32, // in radian
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
            dist: 3.0,
            theta: PI / 9.0,
        };

        let win_dim = renderer.win.size();
        renderer.pen.position = point![0.0, -(win_dim.y as f32 / 2.0)];
        renderer.pen.origin_position = point![0.0, -(win_dim.y as f32 / 2.0)];
        renderer.win.set_light(Light::StickToCamera);
        renderer
    }

    pub fn render(&mut self) {
        let mut system = QuadraticKochIsland::new();
        let sequence = system.get_step(7);
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
                '[' => self.pen.save_state(),
                ']' => self.pen.load_last_state(),
                // skipping unknown characters but keeping it since
                // they can be used for example in node rewriting.
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