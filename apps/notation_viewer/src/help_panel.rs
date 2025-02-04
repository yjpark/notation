use bevy::prelude::*;
use tab_viewer::edger_bevy::app::state::AppState;
use tab_viewer::edger_bevy::bevy_egui::EguiContext;

use tab_viewer::notation::args::NotationArgs;
use tab_viewer::prelude::{MarkDownAsset, KbPageId, KbPage, KbPanel, EasyLinkEvent, NotationSettings, EguiContexts};
use tab_viewer::prelude::{NotationState, NotationAssets, NotationTheme};

use tab_viewer::kb::chords_page::ChordsPage;
use tab_viewer::kb::notes_page::NotesPage;
use tab_viewer::kb::markdown_page::MarkDownPage;

use crate::assets::NotationViewerAssets;

#[derive(Clone, Debug, Resource)]
pub struct HelpPanel {
    pub skip_frames: usize,
    pub open_times: usize,
    pub current_page_id: KbPageId,
    pub welcome: MarkDownPage,
    pub notes: NotesPage,
    pub chords: ChordsPage,
    pub usage: MarkDownPage,
}

impl FromWorld for HelpPanel {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<NotationViewerAssets>().unwrap();
        Self {
            skip_frames: 2,
            open_times: 0,
            current_page_id: Self::WELCOME,
            welcome: MarkDownPage::new(assets.get_welcome_path()),
            notes: Default::default(),
            chords: Default::default(),
            usage: MarkDownPage::new(assets.get_usage_path()),
        }
    }
}

impl HelpPanel {
    pub const WELCOME: KbPageId = KbPageId::MarkDown("kb_welcome");
    pub const USAGE: KbPageId = KbPageId::MarkDown("kb_usage");

    pub const LINK_NOTES: &'static str = ":kb:notes";
    pub const LINK_CHORDS: &'static str = ":kb:chords";
}

impl KbPanel for HelpPanel {
    fn get_title(&self) -> &str {
        "Help (F1, H)"
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
            (KbPageId::Notes, "Notes"),
            (KbPageId::Chords, "Chords"),
            (Self::USAGE, "Usage"),
        ]
    }
    fn get_page_mut(&mut self, page_id: KbPageId) -> &mut dyn KbPage {
        match page_id {
            KbPageId::Notes => &mut self.notes as &mut dyn KbPage,
            KbPageId::Chords => &mut self.chords as &mut dyn KbPage,
            Self::USAGE => &mut self.usage as &mut dyn KbPage,
            _ => &mut self.welcome as &mut dyn KbPage,
        }
    }
    fn on_close(&mut self) {
        if self.open_times == 0 && self.current_page_id == Self::WELCOME {
            self.set_current_page_id(KbPageId::Notes);
        }
        self.open_times += 1;
    }
}

impl HelpPanel {
    pub fn help_ui(
        mut egui_ctx: EguiContexts,
        texts: Res<Assets<MarkDownAsset>>,
        app_state: Res<AppState>,
        mut state: ResMut<NotationState>,
        theme: Res<NotationTheme>,
        mut link_evts: EventWriter<EasyLinkEvent>,
        mut help: ResMut<HelpPanel>,
    ) {
        if help.skip_frames > 0 {
            help.skip_frames -= 1;
            return;
        }
        (&mut help).window_ui(&mut egui_ctx, &texts, &app_state, &mut state, &theme, &mut link_evts);
    }
    pub fn handle_link_evts(
        mut index: ResMut<HelpPanel>,
        mut evts: EventReader<EasyLinkEvent>,
    ) {
        for evt in evts.read() {
            (&mut index).handle_link_evt(evt);
        }
    }
    fn handle_link_evt(
        &mut self,
        evt: &EasyLinkEvent,
    ) {
        println!("handle_link_evt {:?}", evt);
        match evt.link.as_str() {
            Self::LINK_NOTES => {
                self.current_page_id = KbPageId::Notes;
            },
            Self::LINK_CHORDS => {
                self.current_page_id = KbPageId::Chords;
            },
            _ => (),
        }
    }}
