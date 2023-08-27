use std::f32::consts::PI;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

use na::{Point2, Point3, point};
use kiss3d::window::Window;
use kiss3d::light::Light;

use crate::lsystem::LSystem;
use crate::config;

const CONFIG_FILE: &str = "config.yaml";

const COLOR: Point3<f32> = point![0.85f32, 0.66f32, 0.23f32];
// const BG_COLOR: Point3<f32> = point![1.0f32, 0.95f32, 0.83f32];

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

// drawer instance
pub struct Drawer {
    /// current position of the drawer
    position: Point2<f32>,
    /// the position from which the drawer has started
    origin_position: Point2<f32>,
    /// orientation of the drawer
    /// in radian
    angle: f32,
    /// the orientation from which the drawer started
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

/// a state which the drawer can reload
pub struct DrawerState {
    position: Point2<f32>,
    angle: f32, // in radian
}

pub enum Error {
    IOError(std::io::Error),
    SerdeError(serde_yaml::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::SerdeError(err) => write!(f, "{}", err),
        }
    }
}

pub struct Renderer {
    /// active window
    win: Window,
    /// the drawing object
    pen: Drawer,
    /// size of each segment
    dist: f32,
    /// all models loaded from configuration
    loaded_models: Vec<config::Model>,
    /// angle change apply at each rotation
    /// in radian
    delta: f32,
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer = Self {
            win: Window::new("L-System"),
            pen: Drawer::new(point![0.0, 0.0], 0.5 * PI),
            dist: 2.5,
            loaded_models: Self::load_config()
                .map_err(|err| eprintln!("Failed to read config: {}", err))
                .unwrap_or_else(|_| vec![]),
            delta: f32::default(),
        };

        let win_dim = renderer.win.size();
        renderer.pen.position = point![0.0, -(win_dim.y as f32 / 2.0)];
        renderer.pen.origin_position = point![0.0, -(win_dim.y as f32 / 2.0)];
        renderer.win.set_light(Light::StickToCamera);
        renderer.win.set_background_color(1.0f32, 0.95f32, 0.83f32);
        renderer
    }

    fn load_config() -> Result<Vec<config::Model>, Error> {
        let file = File::open(CONFIG_FILE)
            .map_err(|err| Error::IOError(err))?;
        serde_yaml::from_reader(file)
            .map_err(|err| Error::SerdeError(err))
    }

    pub fn render(&mut self) {
        let mut system = self.load_system();
        let sequence = system.get_step(7);
        while self.win.render() {
            self.draw(&sequence);
        }
    }

    fn load_system(&mut self) -> LSystem {

        let toto = config::Model {
            axiom: "X".to_string(),
            rules: std::collections::HashMap::from([
                ('X', "F[+X]F[-X]+X".to_string()),
                ('F', "FF".to_string()),
            ]),
            delta: 90.0
        };
        println!("{}", serde_yaml::to_string(&toto).unwrap());

        let model = &self.loaded_models[0];
        self.delta = model.delta;
        // TODO: avoid copy here
        LSystem::new(
            model.axiom.clone(),
            model.rules.clone(),
        )
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
        self.pen.angle = (self.pen.angle + self.delta * rotation.value()) % (PI * 2.0);
    }
}