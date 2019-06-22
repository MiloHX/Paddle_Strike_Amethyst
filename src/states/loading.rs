//=========================
// Import amethyst modules
//=========================
use amethyst::{
    prelude::*,
    assets::{
        ProgressCounter,
    }
};

//======================
// Import other modules
//======================
use derive_new::new;    // for [#derive(new)]. "derive-new: (latest ver)" need to be added to Cargo.toml

//========================
// Import custom modules
//========================
use super::state_event::CustomStateEvent;

//=======================
// Declare loading state
//=======================
//
// Note that if it is not a unit struct (with no fields)
// you cannot directly use it as the parameter of the Application::new() function
// a seperate method (here we use [derive(new)])to return an instance (Self) need to be used
//
// A #[derive(new)] attribute creates a new constructor function for the annotated type.
// That function takes an argument for each field in the type giving a trivial constructor.
// This is useful since as your type evolves you can make the constructor non-trivial
// (and add or remove fields) without changing client code (i.e., without breaking backwards compatibility).
// It is also the most succinct way to initialise a struct or an enum.
#[derive(new)]
pub struct LoadingState {
    // Tracks loaded assets.
    #[new(default)]
    prefab_loading_progress: Option<ProgressCounter>
}
//=============================
// Implement SimpleState trait
//=============================
impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for LoadingState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.prefab_loading_progress = None;
    }
}