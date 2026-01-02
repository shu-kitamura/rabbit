use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CellStyle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub style: CellStyle,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: CellStyle,
        }
    }
}

impl Cell {
    pub fn from_char(ch: char) -> Self {
        Self {
            ch,
            style: CellStyle,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        let len = usize::from(width) * usize::from(height);
        Self {
            width,
            height,
            cells: vec![Cell::default(); len],
        }
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    pub fn get(&self, x: u16, y: u16) -> Result<&Cell, FrameError> {
        let index = self.index(x, y)?;
        self.cells
            .get(index)
            .ok_or(FrameError::CorruptBuffer { index })
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) -> Result<(), FrameError> {
        let index = self.index(x, y)?;
        let slot = self
            .cells
            .get_mut(index)
            .ok_or(FrameError::CorruptBuffer { index })?;
        *slot = cell;
        Ok(())
    }

    fn index(&self, x: u16, y: u16) -> Result<usize, FrameError> {
        if x >= self.width || y >= self.height {
            return Err(FrameError::OutOfBounds {
                x,
                y,
                width: self.width,
                height: self.height,
            });
        }
        Ok(usize::from(y) * usize::from(self.width) + usize::from(x))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameError {
    OutOfBounds {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    },
    CorruptBuffer {
        index: usize,
    },
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::OutOfBounds {
                x,
                y,
                width,
                height,
            } => write!(
                f,
                "out of bounds: x={x} y={y} for frame size {width}x{height}"
            ),
            FrameError::CorruptBuffer { index } => {
                write!(f, "corrupt frame buffer: index {index} is missing")
            }
        }
    }
}

impl std::error::Error for FrameError {}
