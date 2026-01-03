use std::thread;

use crate::actor::Actor;
use crate::error::Result;
use crate::frame::Frame;
use crate::rabbit::{Rabbit, Sprite};
use crate::renderer::Renderer;
use crate::time::Clock;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub fps: u32,
    pub width: u16,
    pub height: u16,
    pub animal: AnimalKind,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            fps: 30,
            width: 80,
            height: 24,
            animal: AnimalKind::Rabbit,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimalKind {
    Rabbit,
}

pub struct App {
    config: AppConfig,
    renderer: Renderer,
    actor: Box<dyn Actor>,
    clock: Clock,
}

impl App {
    pub fn new(config: AppConfig) -> Result<Self> {
        let renderer = Renderer::new((config.width, config.height));
        let clock = Clock::from_fps(config.fps)?;

        let actor: Box<dyn Actor> = match config.animal {
            AnimalKind::Rabbit => {
                let sprite = default_rabbit_sprite((config.width, config.height))?;
                Box::new(Rabbit::new((0, 0), (1, 0), sprite))
            }
        };

        Ok(Self {
            config,
            renderer,
            actor,
            clock,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let (width, height) = (self.config.width, self.config.height);
        let mut frame = Frame::new(width, height);

        let mut out = std::io::stdout().lock();
        use std::io::Write;
        out.write_all(b"\x1b[2J\x1b[H")?;

        loop {
            let dt = self.clock.tick();
            self.actor.update(dt);

            frame.clear();
            self.actor.render(&mut frame);
            self.renderer.render_to(&mut out, &frame)?;

            thread::sleep(self.clock.frame_duration());
        }
    }
}

fn default_rabbit_sprite(frame_size: (u16, u16)) -> Result<Sprite> {
    let raw_lines = crate::ascii::parse_ascii_art(include_str!("../rabbit.txt"));
    let (raw_w, raw_h) = crate::ascii::dimensions(&raw_lines);

    let frame_w = usize::from(frame_size.0);
    let frame_h = usize::from(frame_size.1);

    let max_w = (frame_w * 7 / 8).max(1);
    let max_h = (frame_h * 7 / 8).max(1);
    let (out_w, out_h) = crate::ascii::fit_dimensions((raw_w, raw_h), (max_w, max_h));

    let frame = crate::ascii::downsample(&raw_lines, out_w, out_h);
    Ok(Sprite::new(vec![frame])?)
}
