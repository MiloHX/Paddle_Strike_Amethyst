//=========================
// Import amethyst modules
//=========================
use amethyst::{
    prelude::*,
    assets::{
        ProgressCounter,
        Loader
    },
    ui::{
        TtfFormat,
        UiTransform,
        Anchor,
        UiText
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
//=======================
// Implement State trait
//=======================
impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.prefab_loading_progress = None;

        //-----------------------
        // Initialize disclaimer
        //-----------------------
        initialize_disclaimer(data.world);
    }

    fn update(&mut self, data: StateData<'_, GameData<'a, 'a>>)-> Trans<GameData<'a, 'a>, CustomStateEvent> {
        data.data.update(&data.world);
        Trans::None
    }

}

//=======================
// initialize disclaimer 
//=======================
//
// Doing this manually for testing, but it should be configured using
// the UI features of Amethyst using .ron file.
fn initialize_disclaimer(world: &mut World) {
    // load font. Note that "load" method is returning a undefined type,
    // you have to use variable "font" later to let the rust compiler to infer it,
    // unless you explicitly give a return type for "load".
    // make sure the UiBundle is added to the game data and DrawUiDesc is added to the graph,
    // or the program will panic "Tried to fetch a resource, but the resource does not exist."
    let font = world.read_resource::<Loader>().load(
        "assets/fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource()
    );

    // create a transform comoponent for the disclaimer
    let disclaimer_transform = UiTransform::new(
        "disclaimer".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.,
        0.,
        1.,
        600.,
        200.,
    );

    // create a ui text comoponent for the disclaimer
    let disclaimer_ui_text = UiText::new(
        font.clone(),
        "This is the disclaimer!".to_string(),
        [1.0, 1.0, 1.0, 1.0],
        50.,
    );

    // combine the stuff into a disclaimer entity
    world.create_entity()
        .with(disclaimer_transform)
        .with(disclaimer_ui_text)
        .build();
}