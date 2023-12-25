use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;

use kiss3d::event::Action;
use kiss3d::event::Key;
use kiss3d::event::Modifiers;
use kiss3d::event::WindowEvent;
use na::{Point2, Point3, point};
use kiss3d::window::Window;
use kiss3d::light::Light;

use crate::lsystem::LSystem;
use crate::config;
use crate::math;
use crate::error as lsysErr;

const CONFIG_FILE: &str = "config.yaml";

const COLOR: Point3<f32> = point![0.85f32, 0.66f32, 0.23f32];
// const BG_COLOR: Point3<f32> = point![1.0f32, 0.95f32, 0.83f32];

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

pub struct Renderer {
    /// active window
    win: Window,
    /// the drawing object
    pen: Drawer,
    /// size of each segment
    dist: f32,
    /// all models loaded from configuration
    loaded_configs: Vec<config::Model>,
    /// cached generated lsystems
    lsystem: HashMap<String, LSystem>,
    /// angle change apply at each rotation
    /// in radian
    delta: f32,
    /// current drawn model idx
    current_model_idx: usize,
    /// current step you are at
    current_step: usize,
    /// current sequence of character render
    sequence: String,
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer = Self {
            win: Window::new("L-System"),
            pen: Drawer::new(point![0.0, 0.0], 0.5 * PI),
            dist: 10.0,
            loaded_configs: Self::load_config()
                .map_err(|err| eprintln!("Failed to read config: {}", err))
                .unwrap_or_else(|_| vec![]),
            lsystem: HashMap::new(),
            delta: 20.0_f32.to_radians(),
            current_model_idx: 0,
            current_step: 1,
            sequence: "".to_string(),
        };

        let step = renderer.current_step;
        renderer.sequence = renderer.load_system().get_step(step);
        let win_dim = renderer.win.size();
        renderer.pen.position = point![0.0, -(win_dim.y as f32 / 2.0)];
        renderer.pen.origin_position = point![0.0, -(win_dim.y as f32 / 2.0)];
        renderer.win.set_light(Light::StickToCamera);
        renderer.win.set_background_color(1.0f32, 0.95f32, 0.83f32);
        renderer
    }

    fn load_config() -> Result<Vec<config::Model>, lsysErr::Error> {
        let file = File::open(CONFIG_FILE)
            .map_err(|err| lsysErr::Error::IOError(err))?;
        let map: HashMap<String, config::Model> = serde_yaml::from_reader(file)
            .map_err(|err| lsysErr::Error::SerdeError(err))?;
        let mut vec = vec![];
        for (name, mut model) in map {
            model.name = name;
            vec.push(model);
        }
        Ok(vec)
    }

    pub fn process_events(&mut self) {
        for event in self.win.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Right, Action::Press, _) => {
                    self.current_model_idx = (self.current_model_idx + 1) % self.loaded_configs.len();
                    let step = self.current_step;
                    self.sequence = self.load_system().get_step(step);
                }
                WindowEvent::Key(Key::Left, Action::Press, _) => {
                    self.current_model_idx = if self.current_model_idx == 0 {
                        self.loaded_configs.len()
                    } else {
                        self.current_model_idx
                    } - 1;
                    let step = self.current_step;
                    self.sequence = self.load_system().get_step(step);
                }
                WindowEvent::Key(Key::Down, Action::Press, _) => {
                    self.current_step = self.current_step - usize::from(self.current_step > 0);
                    let step = self.current_step;
                    self.sequence = self.load_system().get_step(step);
                }
                WindowEvent::Key(Key::Up, Action::Press, _) => {
                    self.current_step = self.current_step + 1;
                    let step = self.current_step;
                    self.sequence = self.load_system().get_step(step);
                }
                WindowEvent::Key(Key::Subtract, Action::Press, modifier) => {
                    if modifier == Modifiers::Control {
                        self.dist -= f32::from(self.dist > 0.0);
                    }
                    if modifier == Modifiers::Alt && self.delta >= 0.0 {
                        self.delta -= 1.0_f32.to_radians()
                    }
                }
                WindowEvent::Key(Key::Add, Action::Press, modifier) => {
                    if modifier == Modifiers::Control {
                        self.dist += 1.0;
                    }
                    if modifier == Modifiers::Alt && self.delta < 360.0_f32.to_radians() {
                        self.delta += 1.0_f32.to_radians()
                    }
                }
                WindowEvent::Key(Key::R, Action::Press, _) => {
                    self.load_system().reset();
                    let step = self.current_step.clone();
                    self.sequence = self.load_system().get_step(step);
                }
                _ => {},
            }
        }
    }

    pub fn render(&mut self) {
        while self.win.render() {
            self.process_events();
            self.draw();
        }
    }

    fn load_system(&mut self) -> &mut LSystem {
        let model = &self.loaded_configs[self.current_model_idx];
        self.delta = model.delta.to_radians();
        if self.lsystem.contains_key(&model.name) {
            self.lsystem.get_mut(&model.name).unwrap()
        } else {
            let new_system = LSystem::new(
                model.axiom.clone(),
                model.rules.clone(),
            );
            self.lsystem.insert(model.name.clone(), new_system);
            self.lsystem.get_mut(&model.name).unwrap()
        }
    }

    fn draw(&mut self) {
        // reset pen position
        self.pen.reset();
        let seq = self.sequence.clone(); // FIXME: find a way to remove this clone
        for c in seq.chars() {
            match c {
                'F' => self.draw_line(),
                'f' => self.move_pen(),
                '+' => self.rotate(math::Rotation::ClockWise),
                '-' => self.rotate(math::Rotation::AntiClockWise),
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

    fn rotate(&mut self, rotation: math::Rotation) {
        self.pen.angle = (self.pen.angle + self.delta * rotation.value()) % (PI * 2.0);
    }
}