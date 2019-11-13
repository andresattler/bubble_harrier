use super::SharedWindow;
use kiss3d::event::*;
use specs::prelude::*;
use std::collections::BTreeSet;

#[derive(Clone, Default, Debug)]
pub struct CurrentInput {
    pub keys: BTreeSet<Key>,
}

pub struct InputSystem {
    win: SharedWindow,
}

impl InputSystem {
    pub fn new(win: SharedWindow) -> Self {
        Self { win }
    }

    pub fn name() -> String {
        "kiss::input_system".to_owned()
    }
}

impl<'s> specs::System<'s> for InputSystem {
    type SystemData = (Write<'s, CurrentInput>);

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
