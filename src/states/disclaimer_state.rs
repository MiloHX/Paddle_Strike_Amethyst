//================
// Import modules
//================

// amethyst modules
use amethyst::{
    ecs::Entity,
    prelude::*,
};

// local modules
use crate::components::flashing_comp::FlashingStyle;
use crate::resources::ui_prefab_registry::UiPrefabRegistry;
use crate::resources::ui_helper::impl_flashing_comp;

//===========
// Constants
//===========
const DISCLAIMER_ID: &str = "disclaimer";
const HELP_MESSAGE:  &str = "help_message";

//=========================
// Define disclaimer state
//=========================
#[derive(Default)]
pub struct DisclaimerState {
    // Loading screen entity
    disclaimer_screen:      Option<Entity>,
    disclaimer_is_ready:    bool,
}

impl SimpleState for DisclaimerState {
    fn on_start(&mut self, data: StateData<GameData>) {
        // assume UiPrefab loading has happened in a previous state
        // look through the UiPrefabRegistry for the "disclaimer" prefab and instantiate it
        let disclaimer_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, DISCLAIMER_ID);
        if let Some(disclaimer_prefab) = disclaimer_prefab {
            self.disclaimer_screen = Some(data
                .world
                .create_entity()
                .with(disclaimer_prefab)
                .build()
            );
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.disclaimer_is_ready = false;
        if let Some(disclaimer_screen) = self.disclaimer_screen {
            if data.world.delete_entity(disclaimer_screen).is_ok() {
                self.disclaimer_screen = None;
            }
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if !self.disclaimer_is_ready {
            if !self.disclaimer_screen.is_none() {
                impl_flashing_comp(
                    HELP_MESSAGE, 
                    data, 
                    true, 
                    1., 
                    0.8, 
                    FlashingStyle::Darkening, 
                    [1., 1., 0., 0.]
                );
                self.disclaimer_is_ready = true;
            }
        }
        Trans::None
    }
}
