use super::SharedWindow;
use crate::bundles::sim::Score;
use crate::util::*;
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
    type SystemData = (Read<'s, Score>);

    fn run(&mut self, score: Self::SystemData) {
        let font = Font::default();
        self.win.borrow_mut().draw_text(
            &format!("Score: {}", *score),
            &Point2::origin(),
            120.0,
            &font,
            &Point::new(1.0, 1.0, 1.0),
        );
    }
}
