pub mod actor;
pub mod app;
pub mod error;
pub mod frame;
pub mod rabbit;
pub mod renderer;
pub mod time;

pub use app::{AnimalKind, App, AppConfig};

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::frame::{Cell, Frame};
    use crate::rabbit::{Rabbit, Sprite};
    use crate::time::Clock;

    #[test]
    fn frame_new_sets_size_and_default() {
        let frame = Frame::new(3, 2);
        assert_eq!(frame.size(), (3, 2));
        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(frame.get(x, y).unwrap(), &Cell::default());
            }
        }
    }

    #[test]
    fn frame_set_get_in_bounds() {
        let mut frame = Frame::new(2, 2);
        let cell = Cell::from_char('R');
        frame.set(1, 1, cell.clone()).unwrap();
        assert_eq!(frame.get(1, 1).unwrap(), &cell);
    }

    #[test]
    fn frame_set_out_of_bounds_returns_err() {
        let mut frame = Frame::new(2, 2);
        let cell = Cell::from_char('R');
        assert!(frame.set(10, 10, cell).is_err());
    }

    #[test]
    fn frame_clear_resets_cells() {
        let mut frame = Frame::new(2, 2);
        frame.set(0, 0, Cell::from_char('R')).unwrap();
        frame.clear();
        assert_eq!(frame.get(0, 0).unwrap(), &Cell::default());
    }

    #[test]
    fn sprite_new_rejects_empty_frames() {
        let frames: Vec<Vec<String>> = vec![];
        assert!(Sprite::new(frames).is_err());
    }

    #[test]
    fn sprite_advance_wraps() {
        let frames = vec![vec!["R".to_string()], vec!["r".to_string()]];
        let mut sprite = Sprite::new(frames).unwrap();
        let first = sprite.current_frame().to_vec();
        sprite.advance();
        sprite.advance();
        assert_eq!(sprite.current_frame(), first.as_slice());
    }

    #[test]
    fn rabbit_update_zero_velocity_no_move() {
        let frames = vec![vec!["R".to_string()]];
        let sprite = Sprite::new(frames).unwrap();
        let mut rabbit = Rabbit::new((1, 1), (0, 0), sprite);
        rabbit.update(Duration::from_secs(1));
        assert_eq!(rabbit.position(), (1, 1));
    }

    #[test]
    fn rabbit_render_draws_sprite_at_position() {
        let frames = vec![vec!["R".to_string()]];
        let sprite = Sprite::new(frames).unwrap();
        let rabbit = Rabbit::new((0, 0), (0, 0), sprite);
        let mut frame = Frame::new(1, 1);
        rabbit.render(&mut frame);
        assert_eq!(frame.get(0, 0).unwrap(), &Cell::from_char('R'));
    }

    #[test]
    fn clock_frame_duration_from_fps() {
        let clock = Clock::from_fps(60).unwrap();
        let dur = clock.frame_duration();
        assert_eq!(dur, Duration::from_nanos(1_000_000_000u64 / 60));
    }
}
