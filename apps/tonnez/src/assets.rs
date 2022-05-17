use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_asset_loader::DynamicAssets;
use notation_bevy::prelude::{bevy_asset_loader, AssetCollection, MarkDownAsset, ExtraAssets, NotationSettings};

#[derive(AssetCollection)]
pub struct TonnezAssets {
}

impl ExtraAssets for TonnezAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        vec![
        ]
    }
    fn setup_extra_keys(settings: &NotationSettings, asset_keys: &mut DynamicAssets) {
    }
}