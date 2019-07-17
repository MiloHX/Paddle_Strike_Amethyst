use amethyst::{
    core::timing::Time,
    prelude::*,
    ecs::Entity,
    ui::UiFinder,
};

use crate::resources::ui_prefab_registry::UiPrefabRegistry;
use crate::resources::ui_helper::{
    impl_flashing_comp,
    set_text_flashing_status,
};
use crate::mx_utils::mx_timer::MxTimer;

//===========
// Constants
//===========
const ARCADE_GAME_UI:   &str = "arcade_game_ui";
const STAGE_INFO:       &str = "stage_info";    

//===================
// Define menu state
//===================
#[derive(Default)]
pub struct ArcadeGameState {
    game_ui:            Option<Entity>,
    game_is_ready:      bool,
    stage_info:         Option<Entity>,
    transition_timer:   MxTimer,
}

impl SimpleState for ArcadeGameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let game_ui_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, ARCADE_GAME_UI);
        if let Some(game_ui_prefab) = game_ui_prefab {
            self.game_ui = Some(data
                .world
                .create_entity()
                .with(game_ui_prefab)
                .build()
            );
        }
        self.transition_timer.set(3.5, false);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.game_is_ready = false;
        if let Some(game_ui) = self.game_ui {
            if data.world.delete_entity(game_ui).is_ok() {
                self.stage_info = None;
                self.game_ui    = None;
            }
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);

        if !self.game_is_ready {
            if self.game_ui.is_some() {
                self.stage_info = data.world.exec(|ui_finder: UiFinder<'_>| {
                    ui_finder.find(STAGE_INFO) 
                }); 
                self.game_is_ready = true;
                if let Some(stage_info) = self.stage_info {   
                    impl_flashing_comp(&stage_info, data, [1., 1., 0., 1.], true, 0.2);
                }
                self.transition_timer.start();
            }
            self.game_is_ready = true;
        } else if self.transition_timer.update(&*data.world.read_resource::<Time>()) {
            if let Some(stage_info) = self.stage_info {  
                set_text_flashing_status(&stage_info, data, false, true);
            }
        }

        Trans::None
    }
}
