use super::SharedWindow;
use crate::{components::*, resources::*, util::*};
use kiss3d::text::Font;
use nalgebra::Point2;
use specs::prelude::*;
use std::rc::Rc;

pub struct UiSystem {
    win: SharedWindow,
    font: Rc<Font>,
}

impl UiSystem {
    pub fn new(win: SharedWindow) -> Self {
        let font = Font::from_bytes(include_bytes!("../../../assets/fonts/manaspc.ttf")).unwrap();
        Self { win, font }
    }

    #[allow(dead_code)]
    pub fn name() -> &'static str {
        "kiss::ui_system"
    }
}

impl<'s> specs::System<'s> for UiSystem {
    type SystemData = (
        Read<'s, Score>,
        ReadExpect<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Vel>,
        ReadStorage<'s, Health>,
    );

    fn run(&mut self, (score, pent, trans, vels, healths): Self::SystemData) {
        let phealth = healths.get(**pent).unwrap();
        let ptrans = trans.get(**pent).unwrap();
        let pvel = vels.get(**pent).unwrap();
        let stats = [
            format!("Score: {}", *score),
            format!("Trans: {:?}", ptrans),
            format!("Vel: {:?}", pvel),
        ]
        .join("\n");
        self.win.borrow_mut().draw_text(
            &stats,
            &Point2::origin(),
            40.0,
            &self.font,
            &Point::new(1.0, 1.0, 1.0),
        );
        let win_height = self.win.borrow().height() as f32;
        let win_width = self.win.borrow().width() as f32;
        let mut health_str = String::from("");
        for _ in 0..phealth.current {
            health_str.push_str(".")
        }
        self.win.borrow_mut().draw_text(
            &health_str,
            &Point2::new(win_width * 1.5, win_height * 1.8),
            120.0,
            &self.font,
            &Point::new(1.0, 1.0, 1.0),
        );
    }
}
