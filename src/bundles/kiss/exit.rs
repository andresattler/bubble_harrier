use super::SharedWindow;

pub struct ExitSystem {
    win: SharedWindow,
}

impl ExitSystem {
    pub fn new(win: SharedWindow) -> Self {
        Self { win }
    }
}

impl<'s> specs::System<'s> for ExitSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        let mut window = self.win.borrow_mut();
        if window.should_close() {
            window.close();
            std::process::exit(0);
        }
    }
}
