mod config;

use crate::config::Configuration;
use config::*;
use specs::prelude::*;
use specs_bundler::{Bundle, Bundler};
use std::fs::read_to_string;
use toml::from_str;

#[derive(Debug, Default)]
pub struct CoreBundle<'deps> {
    deps: Vec<&'deps str>,
}

impl<'deps, 'world, 'a, 'b> Bundle<'world, 'a, 'b> for CoreBundle<'deps> {
    type Error = ();

    #[inline]
    fn bundle(
        self,
        mut bundler: Bundler<'world, 'a, 'b>,
    ) -> Result<Bundler<'world, 'a, 'b>, Self::Error> {
        load_config(&mut bundler.world);
        bundler.dispatcher_builder =
            bundler
                .dispatcher_builder
                .with(ConfigSystem::default(), ConfigSystem::name(), &[]);
        Ok(bundler)
    }
}

fn load_config(world: &mut World) {
    let asset_path = "assets/config.toml";
    let config_str = read_to_string(asset_path).expect("Unable to load config!");
    let configuration: Configuration = from_str(&config_str).expect("Unable to parse config.");
    world.insert(configuration);
}
