// amethyst modules
use amethyst::{
    ecs::Entity,
    prelude::*,
    input::InputEvent,
};

// local modules
use crate::components::flashing_comp::FlashingStyle;
use crate::resources::ui_prefab_registry::UiPrefabRegistry;
use crate::resources::ui_helper::impl_flashing_comp;


//===========
// Constants
//===========
const MAIN_MENU_ID: &str = "main_menu";
const PLACEHOLDER:  &str = "placeholder";

//===================
// Define menu state
//===================
#[derive(Default)]
pub struct MainMenuState {
    // Loading screen entity
    main_menu_screen:      Option<Entity>,
    main_menu_is_ready:    bool,
}

impl SimpleState for MainMenuState {

    fn on_start(&mut self, data: StateData<GameData>) {
        // assume UiPrefab loading has happened in a previous state
        // look through the UiPrefabRegistry for the "disclaimer" prefab and instantiate it
        let main_menu_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, MAIN_MENU_ID);
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
            }
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if !self.main_menu_is_ready {
            if !self.main_menu_screen.is_none() {
                impl_flashing_comp(
                    PLACEHOLDER, 
                    data, 
                    true, 
                    1., 
                    0.8, 
                    FlashingStyle::Darkening, 
                    [1., 1., 0., 0.]
                );
                self.main_menu_is_ready = true;
            }
        }
        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(input_event) => {
                if let InputEvent::ActionPressed(action) = input_event {
                    if action == "confirm" {
                        return Trans::Quit;
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