use std::time::Duration;

use crate::frame::Frame;

pub trait Actor {
    fn update(&mut self, dt: Duration);
    fn render(&self, frame: &mut Frame);
}
