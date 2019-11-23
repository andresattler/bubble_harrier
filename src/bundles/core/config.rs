use crate::config::Configuration;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use specs::prelude::*;
use std::fs::read_to_string;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use toml::from_str;

pub struct ConfigSystem {
    #[allow(unused)]
    watcher: RecommendedWatcher,
    events: Receiver<notify::DebouncedEvent>,
}

impl ConfigSystem {
    pub fn name() -> &'static str {
        "core::config_system"
    }
}

impl Default for ConfigSystem {
    fn default() -> Self {
        let asset_path = "assets/config.toml";
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher =
            Watcher::new(tx, Duration::from_secs(2)).expect("Unable to init watcher.");
        watcher
            .watch(asset_path, RecursiveMode::NonRecursive)
            .expect("Unable to watch config file.");
        Self {
            watcher,
            events: rx,
        }
    }
}

impl<'s> System<'s> for ConfigSystem {
    type SystemData = Read<'s, LazyUpdate>;

    fn run(&mut self, updater: Self::SystemData) {
        if self.events.recv_timeout(Duration::from_millis(10)).is_ok() {
            debug!("Config changed.");
            updater.exec_mut(|world| {
                debug!("Reloading Config file...");
                load_config(world);
            });
        }
    }
}

pub fn load_config(world: &mut World) {
    let asset_path = "assets/config.toml";
    let config_str = read_to_string(asset_path).expect("Unable to load config!");
    let configuration: Configuration = from_str(&config_str).expect("Unable to parse config.");
    world.insert(configuration);
}
