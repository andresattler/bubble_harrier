mod camera;
mod draw;
mod exit;
mod input;
mod setup;
mod ui;

use crate::{components::NodeBuilder, resources::CurrentInput};
use camera::CameraSystem;
use draw::DrawSystem;
use exit::ExitSystem;
use input::InputSystem;
use kiss3d::window::Window;
use specs::prelude::*;
use specs_bundler::{Bundle, Bundler};
use std::cell::RefCell;
use std::rc::Rc;
use ui::UiSystem;

type SharedWindow = Rc<RefCell<Window>>;

#[derive(Debug, Default)]
pub struct KissBundle<'deps> {
    deps: Vec<&'deps str>,
}

impl<'deps, 'world, 'a, 'b> Bundle<'world, 'a, 'b> for KissBundle<'deps> {
    type Error = ();

    #[inline]
    fn bundle(
        self,
        mut bundler: Bundler<'world, 'a, 'b>,
    ) -> Result<Bundler<'world, 'a, 'b>, Self::Error> {
        let shared_window = Rc::new(RefCell::new(setup::window()));
        bundler.world.register::<NodeBuilder>();
        bundler.world.insert(CurrentInput::default());
        bundler.dispatcher_builder = bundler
            .dispatcher_builder
            .with_thread_local(InputSystem::new(shared_window.clone()))
            .with_thread_local(UiSystem::new(shared_window.clone()))
            .with_thread_local(DrawSystem::new(shared_window.clone()))
            .with_thread_local(CameraSystem::new(shared_window.clone()))
            .with_thread_local(ExitSystem::new(shared_window.clone()));
        Ok(bundler)
    }
}
