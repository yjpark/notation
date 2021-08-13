use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::LyricWord;

use crate::prelude::{EntryData, LyonShape, LyonShapeOp, NotationTheme, SingleBundle};

pub type WordText = SingleBundle<LyricWord>;

pub type WordTextData = EntryData<LyricWord>;

pub struct WordTextShape<'a> {
    theme: &'a NotationTheme,
    data: WordTextData,
}

impl<'a> LyonShape<shapes::Line> for WordTextShape<'a> {
    fn get_name(&self) -> String {
        format!("{}:{}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Line {
        let width = self.theme.grid.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.tied_units.0
            - self.theme.lyrics.word_gap;
        shapes::Line(Vec2::ZERO, Vec2::new(width, 0.0))
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.lyrics.line_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.lyrics.line_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.in_bar_pos.0;
        let y = 0.0;
        Transform::from_xyz(x, y, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, WordTextData, shapes::Line, WordTextShape<'a>> for WordTextShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: WordTextData) -> WordTextShape<'a> {
        WordTextShape::<'a> { theme, data }
    }
}
