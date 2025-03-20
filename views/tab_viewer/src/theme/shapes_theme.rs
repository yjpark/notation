use notation_model::prelude::ProtoEntry;
use serde::{Deserialize, Serialize};

use edger_bevy::bevy::{prelude::*, sprite::Anchor};

use crate::prelude::NotationAssets;
use super::theme_colors::hex_linear;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ShapesTheme {
    pub shape_x: f32,
    pub shape_y: f32,
    pub shape_z: f32,
    pub shape_scale: f32,
    pub shape_color: Color,
    pub shape_line_width: f32,
    pub shape_barre_width: f32,
    pub shape_barre_height: f32,
    pub shape_barre_offset_x: f32,
    pub shape_barre_offset_y: f32,
    pub shape_finger_radius: f32,
    pub shape_finger_color: Color,
    pub shape_finger_mute_color: Color,
    pub shape_string_space: f32,
    pub shape_fret_space: f32,
    pub shape_finger_offset_x: f32,
    pub shape_finger_offset_y: f32,
    pub shape_font_size: f32,
    pub shape_font_color: Color,
    pub shape_text_x: f32,
    pub shape_text_y: f32,
    pub shape_text_z: f32,
    pub barre_font_size: f32,
    pub barre_font_color: Color,
    pub barre_text_x: f32,
    pub barre_text_y: f32,
    pub barre_text_z: f32,
}

impl Default for ShapesTheme {
    fn default() -> Self {
        Self {
            shape_x: 12.0,
            shape_y: -12.0,
            shape_z: 11.0,
            shape_scale: 0.75,
            shape_color: hex_linear("F27D7A"),
            shape_line_width: 1.5,
            shape_barre_width: 44.0,
            shape_barre_height: 6.0,
            shape_barre_offset_x: 10.0,
            shape_barre_offset_y: 14.0,
            shape_finger_radius: 3.5,
            shape_finger_color: hex_linear("F27D7A"),
            shape_finger_mute_color: hex_linear("000000"),
            shape_string_space: 7.0,
            shape_fret_space: 12.0,
            shape_finger_offset_x: 27.0,
            shape_finger_offset_y: 14.0,
            shape_font_size: 24.0,
            shape_font_color: hex_linear("F27D7A"),
            shape_text_x: 36.0,
            shape_text_y: -28.0,
            shape_text_z: 1.0,
            barre_font_size: 20.0,
            barre_font_color: hex_linear("F27D7A"),
            barre_text_x: 36.0,
            barre_text_y: 6.0,
            barre_text_z: 1.0,
        }
    }
}

impl ShapesTheme {
    pub fn insert_shape_text(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        text: &String,
    ) {
        let mut entity_commands = commands.spawn_empty();
        let font = assets.latin_font.clone();
        let text_font = TextFont {
            font,
            font_size: self.shape_font_size,
            ..Default::default()
        };
        let justify = JustifyText::Left;
        let anchor = Anchor::Center;
        let shape_text = ProtoEntry::trim_comments(text);
        let text_entity = commands.spawn((
            Text2d::new(shape_text.as_str()),
            TextLayout::new_with_justify(justify),
            text_font,
            TextColor::from(self.shape_font_color),
            Transform::from_xyz(self.shape_text_x, self.shape_text_y, self.shape_text_z),
            anchor,
        )).id();
        commands.entity(entity).add_children(&[text_entity]);
    }
    pub fn insert_barre_text(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        barre: u8,
    ) {
        let mut entity_commands = commands.spawn_empty();
        let font = assets.latin_font.clone();
        let text_font = TextFont {
            font,
            font_size: self.barre_font_size,
            ..Default::default()
        };
        let justify = JustifyText::Left;
        let anchor = Anchor::Center;
        let text_entity = commands.spawn((
            Text2d::new(barre.to_string()),
            TextLayout::new_with_justify(justify),
            text_font,
            TextColor::from(self.barre_font_color),
            Transform::from_xyz(self.barre_text_x, self.barre_text_y, self.barre_text_z),
            anchor,
        )).id();
        commands.entity(entity).add_children(&[text_entity]);
    }
}
