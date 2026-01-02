use std::fmt;
use std::time::Duration;

use crate::actor::Actor;
use crate::frame::{Cell, Frame};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sprite {
    frames: Vec<Vec<String>>,
    current: usize,
}

impl Sprite {
    pub fn new(frames: Vec<Vec<String>>) -> Result<Self, SpriteError> {
        if frames.is_empty() {
            return Err(SpriteError::EmptyFrames);
        }
        Ok(Self { frames, current: 0 })
    }

    pub fn current_frame(&self) -> &[String] {
        &self.frames[self.current]
    }

    pub fn advance(&mut self) {
        self.current = (self.current + 1) % self.frames.len();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpriteError {
    EmptyFrames,
}

impl fmt::Display for SpriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpriteError::EmptyFrames => write!(f, "sprite frames must not be empty"),
        }
    }
}

impl std::error::Error for SpriteError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rabbit {
    pos: (i16, i16),
    pos_fp: (i128, i128),
    velocity: (i16, i16),
    sprite: Sprite,
}

impl Rabbit {
    pub fn new(pos: (i16, i16), velocity: (i16, i16), sprite: Sprite) -> Self {
        let pos_fp = (i128::from(pos.0) * NANOCELLS, i128::from(pos.1) * NANOCELLS);
        Self {
            pos,
            pos_fp,
            velocity,
            sprite,
        }
    }

    pub fn position(&self) -> (i16, i16) {
        self.pos
    }

    pub fn update(&mut self, dt: Duration) {
        let dt_nanos = dt.as_nanos();
        if dt_nanos > 0 {
            let dt_nanos = dt_nanos as i128;
            self.pos_fp.0 += i128::from(self.velocity.0) * dt_nanos;
            self.pos_fp.1 += i128::from(self.velocity.1) * dt_nanos;

            self.pos.0 = clamp_to_i16(self.pos_fp.0 / NANOCELLS);
            self.pos.1 = clamp_to_i16(self.pos_fp.1 / NANOCELLS);

            self.sprite.advance();
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let (frame_width, frame_height) = frame.size();
        let (origin_x, origin_y) = self.pos;

        for (dy, line) in self.sprite.current_frame().iter().enumerate() {
            let y = origin_y as i32 + dy as i32;
            if y < 0 || y >= frame_height as i32 {
                continue;
            }

            for (dx, ch) in line.chars().enumerate() {
                if ch == ' ' {
                    continue;
                }
                let x = origin_x as i32 + dx as i32;
                if x < 0 || x >= frame_width as i32 {
                    continue;
                }
                let _ = frame.set(x as u16, y as u16, Cell::from_char(ch));
            }
        }
    }
}

impl Actor for Rabbit {
    fn update(&mut self, dt: Duration) {
        Rabbit::update(self, dt);
    }

    fn render(&self, frame: &mut Frame) {
        Rabbit::render(self, frame);
    }
}

const NANOCELLS: i128 = 1_000_000_000;

fn clamp_to_i16(value: i128) -> i16 {
    if value > i128::from(i16::MAX) {
        i16::MAX
    } else if value < i128::from(i16::MIN) {
        i16::MIN
    } else {
        value as i16
    }
}
