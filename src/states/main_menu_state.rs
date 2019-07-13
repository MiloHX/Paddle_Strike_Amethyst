// amethyst modules
use amethyst::{
    ecs::Entity,
    prelude::*,
    input::InputEvent,
    ui::UiFinder,
};

// local modules
use crate::components::ui_glowing_comp::UiGlowingStyle;
use crate::components::ui_swinging_comp::UiSwingingStyle;
use crate::components::ui_cursor_option_comp::UiCursorOptionStyle;
use crate::resources::ui_prefab_registry::UiPrefabRegistry;
use crate::resources::ui_helper::*;


//===========
// Constants
//===========
const MAIN_MENU:        &str = "main_menu";
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
    main_menu_button_1p:    Option<Entity>,
    main_menu_button_2p:    Option<Entity>,
    main_menu_button_cpu:   Option<Entity>,
    main_menu_button_exit:  Option<Entity>,
    main_menu_is_ready:     bool,
}

impl SimpleState for MainMenuState {

    fn on_start(&mut self, data: StateData<GameData>) {

        // assume UiPrefab loading has happened in a previous state
        // look through the UiPrefabRegistry for the "disclaimer" prefab and instantiate it
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
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.main_menu_is_ready = false;
        if let Some(main_menu_screen) = self.main_menu_screen {
            if data.world.delete_entity(main_menu_screen).is_ok() {
                self.main_menu_button_1p    = None;
                self.main_menu_button_2p    = None;
                self.main_menu_button_cpu   = None;
                self.main_menu_button_exit  = None;
                self.main_menu_cursor       = None;
                self.main_menu_screen       = None;

            }
        }
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
                self.main_menu_button_1p = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(BUTTON_1_PLAYER) 
                });
                if let Some(button_1p) = self.main_menu_button_1p {
                    impl_glowing_comp(
                        &button_1p,
                        data,
                        true, 
                        1., 
                        0.8, 
                        UiGlowingStyle::Lightening, 
                        [1., 1., 0., 0.]
                    );
                    impl_cursor_option_comp(
                        MAIN_MENU,
                        BUTTON_1_PLAYER,
                        &button_1p,
                        data,
                        UiCursorOptionStyle::Glowing,
                    );
                }

                self.main_menu_button_2p = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(BUTTON_2_PLAYERS) 
                });
                if let Some(button_2p) = self.main_menu_button_2p {
                    impl_glowing_comp(
                        &button_2p,
                        data,
                        false, 
                        1., 
                        0.8, 
                        UiGlowingStyle::Lightening, 
                        [1., 1., 0., 0.]
                    );
                    impl_cursor_option_comp(
                        MAIN_MENU,
                        BUTTON_2_PLAYERS,
                        &button_2p,
                        data,
                        UiCursorOptionStyle::Glowing,
                    );
                }

                self.main_menu_button_cpu = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(BUTTON_CPU_V_CPU) 
                });
                if let Some(button_cpu) = self.main_menu_button_cpu {
                    impl_glowing_comp(
                        &button_cpu,
                        data,
                        false, 
                        1., 
                        0.8, 
                        UiGlowingStyle::Lightening, 
                        [1., 1., 0., 0.]
                    );
                    impl_cursor_option_comp(
                        MAIN_MENU,
                        BUTTON_CPU_V_CPU,
                        &button_cpu,
                        data,
                        UiCursorOptionStyle::Glowing,
                    );
                }

                self.main_menu_button_exit = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(BUTTON_EXIT) 
                });
                if let Some(button_exit) = self.main_menu_button_exit {
                    impl_glowing_comp(
                        &button_exit,
                        data,
                        false, 
                        1., 
                        0.8, 
                        UiGlowingStyle::Lightening, 
                        [1., 1., 0., 0.]
                    );
                    impl_cursor_option_comp(
                        MAIN_MENU,
                        BUTTON_EXIT,
                        &button_exit,
                        data,
                        UiCursorOptionStyle::Glowing,
                    );
                }

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
                            (-160., -100.),
                            (-160., -200.),
                            (-160., -350.),
                        ],
                        vec![
                            BUTTON_1_PLAYER.to_string(), 
                            BUTTON_2_PLAYERS.to_string(), 
                            BUTTON_CPU_V_CPU.to_string(), 
                            BUTTON_EXIT.to_string(),
                        ],
                    );
                }

                //--------
                // Title
                //--------
                let rate =0.5;
                let height = 5.;
                let cut_off = 0.9;
                let delay = 0.2;
                if let Some(title_01_p) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_01_P)
                })  {
                    impl_jumping_comp(
                        &title_01_p,
                        data,
                        MAIN_MENU,
                        0,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                } 
                if let Some(title_02_a) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_02_A)
                })  {
                    impl_jumping_comp(
                        &title_02_a,
                        data,
                        MAIN_MENU,
                        1,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }     
                if let Some(title_03_d) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_03_D)
                })  {
                    impl_jumping_comp(
                        &title_03_d,
                        data,
                        MAIN_MENU,
                        2,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                } 
                if let Some(title_04_d) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_04_D)
                })  {
                    impl_jumping_comp(
                        &title_04_d,
                        data,
                        MAIN_MENU,
                        3,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }   
                if let Some(title_05_l) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_05_L)
                })  {
                    impl_jumping_comp(
                        &title_05_l,
                        data,
                        MAIN_MENU,
                        4,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }    
                if let Some(title_06_e) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_06_E)
                })  {
                    impl_jumping_comp(
                        &title_06_e,
                        data,
                        MAIN_MENU,
                        5,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }       
                if let Some(title_07_s) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_07_S)
                })  {
                    impl_jumping_comp(
                        &title_07_s,
                        data,
                        MAIN_MENU,
                        6,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }                    
                if let Some(title_08_t) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_08_T)
                })  {
                    impl_jumping_comp(
                        &title_08_t,
                        data,
                        MAIN_MENU,
                        7,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                } 
                if let Some(title_09_r) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_09_R)
                })  {
                    impl_jumping_comp(
                        &title_09_r,
                        data,
                        MAIN_MENU,
                        8,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }     
                if let Some(title_10_i) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_10_I)
                })  {
                    impl_jumping_comp(
                        &title_10_i,
                        data,
                        MAIN_MENU,
                        9,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }     
                if let Some(title_11_k) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_11_K)
                })  {
                    impl_jumping_comp(
                        &title_11_k,
                        data,
                        MAIN_MENU,
                        10,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }     
                if let Some(title_12_e) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(TITLE_12_E)
                })  {
                    impl_jumping_comp(
                        &title_12_e,
                        data,
                        MAIN_MENU,
                        11,
                        true,
                        rate,
                        height,
                        cut_off,
                        delay,
                    );
                }     
                self.main_menu_is_ready = true;
            }
        }
        Trans::None
    }

    fn handle_event(&mut self, mut data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(input_event) => {
                if let InputEvent::ActionPressed(action) = input_event {
                    if action == "confirm" {
                        if let Some(cursor) = self.main_menu_cursor {
                            let action = get_cursor_action(&cursor, &mut data);
                            if action.eq(BUTTON_1_PLAYER) {
                                return Trans::Quit;
                            } else if action.eq(BUTTON_2_PLAYERS) {
                                return Trans::Quit;
                            } else if action.eq(BUTTON_CPU_V_CPU) {
                                return Trans::Quit;
                            } else if action.eq(BUTTON_EXIT) {
                                return Trans::Quit;
                            }
                        }
                    } else if action == "up" {
                        if let Some(cursor) = self.main_menu_cursor {
                            move_cursor(&cursor, &mut data, false);
                        }
                    } else if action == "down" {
                        if let Some(cursor) = self.main_menu_cursor {
                            move_cursor(&cursor, &mut data, true);
                        }                     
                    }
                    Trans::None
                } else {
                    Trans::None
                }
            },
            _ => Trans::None,
        }
    }

}