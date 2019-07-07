//=========================
// Import amethyst modules
//=========================
use amethyst::{
    prelude::*,
};

//======================
// Import local modules
//======================
use super::state_event::CustomStateEvent;

//==========================
// Declare disclaimer state
//==========================
#[derive(Default)]
pub struct DisclaimerState {}

impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for DisclaimerState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    }

    fn update(&mut self, data: StateData<'_, GameData<'a, 'a>>)-> Trans<GameData<'a, 'a>, CustomStateEvent> {
        data.data.update(&data.world);
        Trans::None
    }

}