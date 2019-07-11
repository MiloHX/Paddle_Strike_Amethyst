// amethyst modules
use amethyst::{
    ecs::Entity,
    prelude::*,
    input::InputEvent,
    ui::UiFinder,
};

// local modules
use crate::components::ui_flashing_comp::UiFlashingStyle;
use crate::components::ui_swinging_comp::UiSwingingStyle;
use crate::resources::ui_prefab_registry::UiPrefabRegistry;
use crate::resources::ui_helper::*;


//===========
// Constants
//===========
const MAIN_MENU:        &str = "main_menu";
const BUTTON_1_PLAYER:  &str = "button_1_player";
const BUTTON_2_PLAYER2: &str = "button_2_players";
const BUTTON_CPU_V_CPU: &str = "button_cpu_v_cpu";
const BUTTON_EXIT:      &str = "button_exit";
const CURSOR:           &str = "cursor";

//===================
// Define menu state
//===================
#[derive(Default)]
pub struct MainMenuState {
    // Loading screen entity
    main_menu_screen:   Option<Entity>,
    main_menu_cursor:   Option<Entity>,
    main_menu_is_ready: bool,
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
                self.main_menu_screen = None;
                self.main_menu_cursor = None;
            }
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);

        // Adding extra components to the menu item
        // Not an ideal solutioin, should be configured in a ron file.
        if !self.main_menu_is_ready {
            if self.main_menu_screen.is_some() {
                impl_flashing_comp(
                    BUTTON_1_PLAYER, 
                    data, 
                    false, 
                    1., 
                    0.8, 
                    UiFlashingStyle::Lightening, 
                    [1., 1., 0., 0.]
                );
                impl_flashing_comp(
                    BUTTON_2_PLAYER2, 
                    data, 
                    false, 
                    1., 
                    0.8, 
                    UiFlashingStyle::Lightening, 
                    [1., 1., 0., 0.]
                );
                impl_flashing_comp(
                    BUTTON_CPU_V_CPU, 
                    data, 
                    false, 
                    1., 
                    0.8, 
                    UiFlashingStyle::Lightening, 
                    [1., 1., 0., 0.]
                );
                impl_flashing_comp(
                    BUTTON_EXIT, 
                    data, 
                    false, 
                    1., 
                    0.8, 
                    UiFlashingStyle::Lightening, 
                    [1., 1., 0., 0.]
                );

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
                    impl_cursor_movement_comp(
                        &cursor_entity,
                        data,                       
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
                        info!("CONFIRM PRESSED!");
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