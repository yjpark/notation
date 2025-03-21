use tab_viewer::edger_bevy::app::state::AppState;
use tab_viewer::edger_bevy::bevy_prelude::*;
use tab_viewer::edger_bevy::bevy_egui::{egui, EguiContext};
use tab_viewer::prelude::{StereoStream, ProtoTab, NotationSettings, Control, MidiState, PlayControlEvent, MidiControl, EguiContexts};

use tab_viewer::prelude::{MarkDownAsset, KbPageId, KbPage, KbContent, KbPanel, DockSide, EasyLinkEvent};
use tab_viewer::prelude::{NotationState, NotationAssets, NotationTheme};

use tab_viewer::kb::markdown_page::MarkDownPage;

use crate::assets::NotationKnowledgeBaseAssets;
use crate::guitar::page::GuitarPage;
use crate::theory::scale::ScalePage;
use crate::theory::sound::{SoundPage, SoundSection};

#[derive(Clone, Debug, Resource)]
pub struct IndexPanel {
    pub skip_frames: usize,
    pub current_page_id: KbPageId,
    pub welcome: MarkDownPage,
    pub sound: SoundPage,
    pub scale: ScalePage,
    pub guitar: GuitarPage,
}

/* if the first page displayed is using the chinese font, will crash, this is a hack around this
wgpu error: Validation Error

Caused by:
In CommandEncoder::copy_buffer_to_texture
Copy error
copy of Y 0..256 would end up overruning the bounds of the Destination texture of Y size 128
TODO: Check whether still needed after upgrade bevy and bevy_egui
    */
impl FromWorld for IndexPanel {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<NotationKnowledgeBaseAssets>().unwrap();
        Self {
            skip_frames: 2,

            #[cfg(debug_assertions)]
            current_page_id: Self::SCALE,
            #[cfg(not(debug_assertions))]
            current_page_id: Self::WELCOME,

            welcome: MarkDownPage::new(assets.get_welcome_path()),
            sound: SoundPage::new(assets.get_sound_path()),
            scale: ScalePage::new(assets.get_scale_path()),
            guitar: GuitarPage::new(assets.get_guitar_path()),
        }
    }
}

impl IndexPanel {
    pub const WELCOME: KbPageId = KbPageId::MarkDown("kb_welcome");
    pub const SOUND: KbPageId = KbPageId::Custom("kb_sound");
    pub const SCALE: KbPageId = KbPageId::Custom("kb_scale");
    pub const GUITAR: KbPageId = KbPageId::Custom("kb_guitar");

    pub const LINK_SOUND: &'static str = ":kb:sound";
    pub const LINK_SCALE: &'static str = ":kb:scale";
    pub const LINK_GUITAR: &'static str = ":kb:guitar";
    pub const LINK_SOUND_SINGLE_STRING: &'static str = ":kb:sound:single_string";

    pub const LINK_MIDI_PLAY: &'static str = ":midi:play";
    pub const LINK_MIDI_STOP: &'static str = ":midi:stop";
}

impl KbPanel for IndexPanel {
    fn get_title(&self) -> &str {
        "Index (F1, H)"
    }

    fn get_current_page_id(&self) -> KbPageId {
        self.current_page_id.clone()
    }

    fn set_current_page_id(&mut self, page_id: KbPageId) {
        self.current_page_id = page_id;
    }

    fn get_page_tabs(&self) -> Vec<(KbPageId, &'static str)> {
        vec![
            (Self::WELCOME, "Welcome"),
            (Self::SOUND, "Sound"),
            (Self::SCALE, "Scale"),
            (Self::GUITAR, "Guitar"),
        ]
    }

    fn get_page_mut(&mut self, page_id: KbPageId) -> &mut dyn KbPage {
        match page_id {
            Self::SOUND => &mut self.sound as &mut dyn KbPage,
            Self::SCALE => &mut self.scale as &mut dyn KbPage,
            Self::GUITAR => &mut self.guitar as &mut dyn KbPage,
            _ => &mut self.welcome as &mut dyn KbPage,
        }
    }
}

impl IndexPanel {
    pub fn check_reload(
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        index: Res<IndexPanel>,
    ) {
        if let Some(tab) = state.tab.as_ref() {
            let need_reload = match index.current_page_id {
                Self::SCALE => {
                    index.scale.check_reload(&tab)
                },
                _ => false,
            };
            if need_reload {
                Control::reload_tab(&mut state, &mut theme);
            }
        }
    }

    pub fn hack_settings(
        app_state: Res<AppState>,
        state: Res<NotationState>,
        mut theme: ResMut<NotationTheme>,
        mut settings: ResMut<NotationSettings>,
        _index: Res<IndexPanel>,
    ) {
        if !state.show_kb {
            return;
        }
        theme.sizes.melody.note_height = 8.0;
        theme.sizes.melody.semitone_height = 8.0;
        theme.texts.melody.text_y = -18.0;
        settings.hide_mini_map = true;
        settings.hide_bar_number = true;
        settings.layout.focus_bar_ease_ms = 0;
        settings.show_note_pitch = true;
        settings.show_note_syllable = true;
        if app_state.window_width > 0.0 && app_state.window_height > 0.0 {
            if app_state.window_width > app_state.window_height {
                let width = app_state.window_width / 3.0 + theme.sizes.layout.page_margin;
                settings.hide_guitar_view = false;
                settings.hide_chords_view = true;
                settings.override_guitar_width = Some(width);
                settings.layout.override_tab_width = None;
            } else {
                settings.hide_guitar_view = true;
                settings.hide_chords_view = false;
                settings.override_guitar_width = None;
                let height = app_state.window_height / 3.0;
                settings.override_chord_size = Some(height);
            }
        }
    }

    pub fn index_ui(
        mut egui_ctx: EguiContexts,
        texts: Res<Assets<MarkDownAsset>>,
        app_state: Res<AppState>,
        mut state: ResMut<NotationState>,
        theme: Res<NotationTheme>,
        mut link_evts: EventWriter<EasyLinkEvent>,
        mut index: ResMut<IndexPanel>,
    ) {
        if index.skip_frames > 0 {
            index.skip_frames -= 1;
            return;
        }
        if app_state.window_width > app_state.window_height {
            let width = app_state.window_width / 3.0;
            (&mut index).side_ui(&mut egui_ctx, &texts, &app_state, &mut state, &theme, &mut link_evts, DockSide::Left, (width, width));
        } else {
            let height = app_state.window_height / 3.0;
            (&mut index).side_ui(&mut egui_ctx, &texts, &app_state, &mut state, &theme, &mut link_evts, DockSide::Top, (height, height));
        }
        (&mut index).content_ui(&mut egui_ctx, &texts, &app_state, &state, &theme, &mut link_evts);
    }

    fn content_ui(
        &mut self,
        egui_ctx: &mut EguiContexts,
        texts: &Assets<MarkDownAsset>,
        app_state: &AppState,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        if let Some(content) = match self.current_page_id {
            Self::SOUND => {
                if self.sound.content_visible() {
                    Some(&mut self.sound as &mut dyn KbContent)
                } else {
                    None
                }
            },
            _ => None,
        } {
            egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
                content.content_ui(ui, texts, app_state, state, theme, link_evts);
            });
        }
    }

    pub fn handle_link_evts(
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut index: ResMut<IndexPanel>,
        mut evts: EventReader<EasyLinkEvent>,
    ) {
        for evt in evts.read() {
            (&mut index).handle_link_evt(&mut midi_state, &mut play_control_evts, evt);
        }
    }

    fn handle_link_evt(
        &mut self,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
        evt: &EasyLinkEvent,
    ) {
        println!("handle_link_evt {:?}", evt);
        match evt.link.as_str() {
            Self::LINK_SOUND => {
                self.current_page_id = Self::SOUND;
            },
            Self::LINK_SCALE => {
                self.current_page_id = Self::SCALE;
            },
            Self::LINK_SOUND_SINGLE_STRING => {
                self.current_page_id = Self::SOUND;
                self.sound.section = SoundSection::SingleString(Default::default());
            },
            Self::LINK_MIDI_PLAY => {
                MidiControl::play(midi_state, play_control_evts)
            },
            Self::LINK_MIDI_STOP => {
                MidiControl::stop(midi_state, play_control_evts)
            },
            _ => (),
        }
    }

    fn _index_audio(
        &mut self,
        stream: &mut StereoStream,
    ) {
        match self.current_page_id {
            Self::SOUND => {
                self.sound.audio(stream);
            },
            _ => {},
        }
    }

    pub fn index_audio(
        mut index: ResMut<IndexPanel>,
        audio_player_query: Query<&AudioPlayer<StereoStream>>,
        mut assets: ResMut<Assets<StereoStream>>,
    ) {
        for audio_player in audio_player_query.iter() {
            if let Some(stream) = assets.get_mut(&audio_player.0) {
                (&mut index)._index_audio(stream);
            }
        }
    }

    pub fn make_tab(&self, _tab_path: String) -> ProtoTab {
        match self.current_page_id {
            Self::SCALE => {
                self.scale.make_tab()
            },
            _ => Self::make_default_tab(),
        }
    }

    pub fn make_default_tab() -> ProtoTab {
        ProtoTab::new_empty()
    }
}
