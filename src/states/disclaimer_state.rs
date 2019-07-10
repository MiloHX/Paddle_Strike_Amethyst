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
    disclaimer_screen: Option<Entity>,
}

impl SimpleState for DisclaimerState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
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
            impl_flashing_comp(
                HELP_MESSAGE, 
                &mut data, 
                true, 
                1., 
                0.8, 
                FlashingStyle::Darkening, 
                [1., 1., 0., 0.]
            );
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(disclaimer_screen) = self.disclaimer_screen {
            if data.world.delete_entity(disclaimer_screen).is_ok() {
                self.disclaimer_screen = None;
            }
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        Trans::None
    }
}
