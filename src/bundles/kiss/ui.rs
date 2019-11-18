use super::SharedWindow;
use crate::{components::*, util::*};
use crate::bundles::sim::Score;
use kiss3d::text::Font;
use nalgebra::Point2;
use specs::prelude::*;

pub struct UiSystem {
    win: SharedWindow,
}

impl UiSystem {
    pub fn new(win: SharedWindow) -> Self {
        Self { win }
    }

    pub fn name() -> &'static str {
        "kiss::ui_system"
    }
}

impl<'s> specs::System<'s> for UiSystem {
    type SystemData = (
        Read<'s, Score>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, ObjectKind>,
        );

    fn run(&mut self, (score, healths, kinds): Self::SystemData) {
        let player = (&healths, &kinds).join().find(|(_,kind)| {
            match kind {
                ObjectKind::Player => true,
                _ => false,
            }
        }).unwrap();
        let font = Font::default();
        self.win.borrow_mut().draw_text(
            &format!("Score: {}", *score),
            &Point2::origin(),
            120.0,
            &font,
            &Point::new(1.0, 1.0, 1.0),
        );
        let win_height = self.win.borrow().height() as f32;
        let win_width = self.win.borrow().width() as f32;
        let mut health_str = String::from("");
        for _ in 0..player.0.current {
            health_str.push_str(".")
        }
        self.win.borrow_mut().draw_text(
            &health_str,
            &Point2::new(win_width * 1.5, win_height * 1.8),
            120.0,
            &font,
            &Point::new(1.0, 1.0, 1.0),
        );
    }
}
