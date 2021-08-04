use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::{GuitarShapesLaneBundle, GuitarStringsLaneBundle};
use notation_model::prelude::Track;

pub struct GuitarPlugin;

impl Plugin for GuitarPlugin {
    fn build(&self, _app: &mut AppBuilder) {
        /*
        app
            .add_system(on_add_guitar_bar.system())
        ;
         */
    }
}

impl GuitarPlugin {
    pub fn insert_guitar_strings_lane_extra(commands: &mut EntityCommands, track: Arc<Track>) {
        commands.insert_bundle(GuitarStringsLaneBundle::new(track));
    }
    pub fn insert_guitar_shapes_lane_extra(commands: &mut EntityCommands, track: Arc<Track>) {
        commands.insert_bundle(GuitarShapesLaneBundle::new(track));
    }
}
