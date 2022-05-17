use notation_bevy::{bevy::prelude::bevy_main, prelude::NotationArgs};
use tonnez::assets::TonnezAssets;

#[bevy_main]
fn main() {
    let args = NotationArgs::parse_args();
    tonnez::app::FretsApp::run::<FretsAssets>(args);
}
