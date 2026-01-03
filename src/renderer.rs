use std::io::{self, Write};

use crate::frame::Frame;

#[derive(Debug)]
pub struct Renderer {
    size: (u16, u16),
    last_frame: Frame,
}

impl Renderer {
    pub fn new(size: (u16, u16)) -> Self {
        Self {
            size,
            last_frame: Frame::new(size.0, size.1),
        }
    }

    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    pub fn render(&mut self, frame: &Frame) -> io::Result<()> {
        let mut out = io::stdout().lock();
        self.render_to(&mut out, frame)
    }

    pub fn render_to<W: Write>(&mut self, out: &mut W, frame: &Frame) -> io::Result<()> {
        let (width, height) = self.size;

        out.write_all(b"\x1b[H")?;
        for y in 0..height {
            for x in 0..width {
                let ch = frame.get(x, y).map(|c| c.ch).unwrap_or(' ');
                let mut buffer = [0u8; 4];
                out.write_all(ch.encode_utf8(&mut buffer).as_bytes())?;
            }
            if y + 1 < height {
                out.write_all(b"\n")?;
            }
        }
        out.flush()?;

        self.last_frame = frame.clone();
        Ok(())
    }
}
