use super::SharedWindow;
use crate::resources::CurrentInput;
use kiss3d::event::*;
use specs::prelude::*;

pub struct InputSystem {
    win: SharedWindow,
}

impl InputSystem {
    pub fn new(win: SharedWindow) -> Self {
        Self { win }
    }

    #[allow(dead_code)]
    pub fn name() -> &'static str {
        "kiss::input_system"
    }
}

impl<'s> specs::System<'s> for InputSystem {
    type SystemData = Write<'s, CurrentInput>;

    fn run(&mut self, mut c_in: Self::SystemData) {
        self.win.borrow().events().iter().for_each(|ev| {
            if let WindowEvent::Key(key, action, _) = ev.value {
                match action {
                    Action::Press => c_in.keys.insert(key),
                    Action::Release => c_in.keys.remove(&key),
                };
            }
        });
    }
}
