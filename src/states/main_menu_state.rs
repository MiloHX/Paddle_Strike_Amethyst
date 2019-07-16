// amethyst modules
use amethyst::{
    core::timing::Time,
    ecs::Entity,
    prelude::*,
    input::InputEvent,
    ui::UiFinder,
};

// local modules
use crate::states::arcade_game_state::ArcadeGameState;
use crate::components::ui_glowing_comp::UiGlowingStyle;
use crate::components::ui_swinging_comp::UiSwingingStyle;
use crate::resources::ui_prefab_registry::UiPrefabRegistry;
use crate::resources::ui_helper::*;
use crate::resources::audio::{
    SoundType, play_sfx, resume_music, pause_music,
};
use crate::mx_utils::mx_timer::MxTimer;


//===========
// Constants
//===========
const MAIN_MENU:        &str = "main_menu";
const BUTTON_ARCADE:    &str = "button_arcade";
const BUTTON_1_PLAYER:  &str = "button_1_player";
const BUTTON_2_PLAYERS: &str = "button_2_players";
const BUTTON_CPU_V_CPU: &str = "button_cpu_v_cpu";
const BUTTON_EXIT:      &str = "button_exit";
const CURSOR:           &str = "cursor";
const TITLE_01_P:       &str = "title_01_p";
const TITLE_02_A:       &str = "title_02_a";
const TITLE_03_D:       &str = "title_03_d";
const TITLE_04_D:       &str = "title_04_d";
const TITLE_05_L:       &str = "title_05_l";
const TITLE_06_E:       &str = "title_06_e";
const TITLE_07_S:       &str = "title_07_s";
const TITLE_08_T:       &str = "title_08_t";
const TITLE_09_R:       &str = "title_09_r";
const TITLE_10_I:       &str = "title_10_i";
const TITLE_11_K:       &str = "title_11_k";
const TITLE_12_E:       &str = "title_12_e";


//===================
// Define menu state
//===================
#[derive(Default)]
pub struct MainMenuState {
    // Loading screen entity
    main_menu_screen:       Option<Entity>,
    main_menu_cursor:       Option<Entity>,
    main_menu_buttons:      Vec<Option<Entity>>,
    main_menu_is_ready:     bool,
    transition_timer:       MxTimer,
    triggered_action:       String,
}

impl SimpleState for MainMenuState {

    fn on_start(&mut self, data: StateData<GameData>) {

        // assume UiPrefab loading has happened in a previous state
        // look through the UiPrefabRegistry for the "main menu" prefab and instantiate it
        let main_menu_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, MAIN_MENU);
        if let Some(main_menu_prefab) = main_menu_prefab {
            self.main_menu_screen = Some(data
                .world
                .create_entity()
                .with(main_menu_prefab)
                .build()
            );
        }
        self.transition_timer.set(2., false);
        self.triggered_action = "".to_string();

        resume_music(data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.main_menu_is_ready = false;
        if let Some(main_menu_screen) = self.main_menu_screen {
            if data.world.delete_entity(main_menu_screen).is_ok() {
                self.main_menu_buttons.clear();
                self.main_menu_cursor       = None;
                self.main_menu_screen       = None;
            }
        }

        pause_music(data.world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);

        // Adding extra components to the menu item
        // Not an ideal solutioin, should be configured in a ron file.
        if !self.main_menu_is_ready {
            if self.main_menu_screen.is_some() {
                //---------
                // Buttons
                //---------
                self.main_menu_buttons = impl_bulk_button(
                    vec![
                        BUTTON_ARCADE,
                        BUTTON_1_PLAYER,
                        BUTTON_2_PLAYERS,
                        BUTTON_CPU_V_CPU,
                        BUTTON_EXIT,
                    ],
                    data,
                    MAIN_MENU,
                    true,                       // is glowing
                    1.,                         // glowing rate
                    0.8,                        // glowing intensity
                    UiGlowingStyle::Lightening, // glowing style
                    [1., 1., 0., 0.],           // rgba factor
                    0.7,                        // flash rate
                );

                //--------
                // Cursor
                //--------
                self.main_menu_cursor = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(CURSOR) 
                });

                if let Some(cursor_entity) = self.main_menu_cursor {
                    impl_swinging_comp(
                        &cursor_entity,
                        data,
                        true,
                        1.5,
                        1.,
                        UiSwingingStyle::Horizontal,
                    );
                    impl_cursor_comp(
                        &cursor_entity,
                        data,
                        MAIN_MENU,
                        vec![
                            (-160., 0.), 
                            (-160., -80.),
                            (-160., -160.),
                            (-160., -240.),
                            (-160., -360.),
                        ],
                        vec![
                            BUTTON_ARCADE,
                            BUTTON_1_PLAYER, 
                            BUTTON_2_PLAYERS, 
                            BUTTON_CPU_V_CPU, 
                            BUTTON_EXIT,
                        ],
                    );
                }

                //--------
                // Title
                //--------
                impl_bulk_waving(
                    vec![
                        TITLE_01_P,
                        TITLE_02_A,
                        TITLE_03_D,
                        TITLE_04_D,
                        TITLE_05_L,
                        TITLE_06_E,
                        TITLE_07_S,
                        TITLE_08_T,
                        TITLE_09_R,
                        TITLE_10_I,
                        TITLE_11_K,
                        TITLE_12_E,
                    ],
                    data,
                    MAIN_MENU,
                    true,
                    1.,     // rate
                    1.,     // height
                    0.,     // high cut
                    1.,     // low cut
                    0.15,   // delay
                    1.3,    // play time
                    5.,     // wait time
                );
 
                self.main_menu_is_ready = true;
            }
        } else if !self.triggered_action.is_empty() {
            let time = data.world.read_resource::<Time>();
            if self.transition_timer.update(&*time) {
                if self.triggered_action == BUTTON_ARCADE {
                    return Trans::Switch(Box::new(ArcadeGameState::default()));
                }
            }
        }
        Trans::None
    }

    fn handle_event(&mut self, mut data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if self.triggered_action.is_empty() {
            match event {
                StateEvent::Input(input_event) => {
                    if let InputEvent::ActionPressed(action) = input_event {
                        if action == "confirm" {
                            // play sfx
                            play_sfx(SoundType::ButtonPush, data.world);

                            // handle option                       
                            if let Some(cursor) = self.main_menu_cursor {
                                let action = get_cursor_action(&cursor, &mut data);
                                self.triggered_action = action.to_string();
                                if action.eq(BUTTON_ARCADE) {
                                    if let Some(button) = self.main_menu_buttons[0] {
                                        flashing_text(&button, &mut data);
                                        freeze_cursor(&cursor, &mut data);
                                        self.transition_timer.start();
                                    }
                                } else if action.eq(BUTTON_1_PLAYER) {
                                    if let Some(button) = self.main_menu_buttons[1] {
                                        flashing_text(&button, &mut data);
                                    }
                                } else if action.eq(BUTTON_2_PLAYERS) {
                                    if let Some(button) = self.main_menu_buttons[2] {
                                        flashing_text(&button, &mut data);
                                    }
                                } else if action.eq(BUTTON_CPU_V_CPU) {
                                    if let Some(button) = self.main_menu_buttons[3] {
                                        flashing_text(&button, &mut data);
                                    }
                                } else if action.eq(BUTTON_EXIT) {
                                    return Trans::Quit;
                                }
                            }
                        } else if action == "ui_up" {
                            if let Some(cursor) = self.main_menu_cursor {
                                move_cursor(&cursor, &mut data, false);
                            }
                        } else if action == "ui_down" {
                            if let Some(cursor) = self.main_menu_cursor {
                                move_cursor(&cursor, &mut data, true);
                            }                     
                        }
                        Trans::None
                    } else {
                        Trans::None
                    }
                },
                _ => Trans::None
            }
        } else {
            Trans::None
        }
    }

}