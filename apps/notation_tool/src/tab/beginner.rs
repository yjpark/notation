use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_1_right_hand() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::G, Scale::Major, Signature::_4_4, Tempo::Bpm(60))
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 3 5 )
                "6-" Chord ( 6: 3- 5 )
            ]}
            {guitar Guitar [
                Fretboard
                $duration = _1
                "Em" Shape ( 0 2 2 0 0 0 )
                "G" Shape ( 3 2 0 0 0 0 )
                $duration = T_1_8
                "picks" Pick [ 6 3 2 1 2 3 ]
                Pick [ 6 3 2 1 2 3 ] |
            ]}
        ]
        Sections: [
            {"A" Verse [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "picks" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "picks" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "picks" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "picks" | ]
                }
            ]}
        ]
        Form: "A" "A"
    }
}
