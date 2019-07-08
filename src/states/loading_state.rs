//=========================
// Import amethyst modules
//=========================
use amethyst::{
    prelude::*,
    assets::{
        Completion, 
        ProgressCounter,
    },
    ui::{
        UiCreator, 
        UiFinder, 
        UiText,
    },
};

//======================
// Import local modules
//======================
use crate::components::FlashingComp;
use crate::states::disclaimer_state::DisclaimerState;
use crate::states::state_event::CustomStateEvent;

//=======================
// Declare loading state
//=======================
//
// Note that if it is not a unit struct (with no fields)
// you cannot directly use it as the parameter of the Application::new() function
// a seperate method (here we use default())to return an instance (Self) need to be used
pub struct LoadingState {
    // Tracks loaded assets.
    // Here we use an Option to allow "None" for this field.
    loading_progress: Option<ProgressCounter>,
}

//=========================
// Implement Default trait
//=========================
impl Default for LoadingState {
    // Define how to return when default() is called
    fn default() -> Self {
        LoadingState {
            loading_progress: Some(ProgressCounter::new()),
        }
    }
}
//=======================
// Implement State trait
//=======================
impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for LoadingState {

    //----------------
    // Start up tasks
    //----------------
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        // initialize disclaimer with a ron file.
        // here we need to handle Some and None case of the loading_progress
        if let Some(counter) = &mut self.loading_progress {
            // Make use of UiCreator to create the UI defined in disclaimer.ron
            data.world.exec(|mut creator: UiCreator<'_>| {
                creator.create("ui/disclaimer.ron", counter);
            });
        } else {
            // None handling
            println!("The loading progress counter is not created correctly! UI loading aborted");
        }

    }

    //--------------
    // Update tasks 
    //--------------
    //
    // Note that this will be called repeatly until transite to other state
    fn update(&mut self, data: StateData<'_, GameData<'a, 'a>>)-> Trans<GameData<'a, 'a>, CustomStateEvent> {

        // update game data
        data.data.update(&data.world);

        // here will get the counter as a reference
        if let Some(ref counter) = self.loading_progress.as_ref() {
            match counter.complete() {
                Completion::Loading  => {
                    // onging
                }
                Completion::Failed   => {
                    println!("======= Loading Failed    =======");
                }
                Completion::Complete => {
                    println!("======= Loading Completed =======");
                    // clear the counter
                    self.loading_progress = None;
                    // use UiFinder to get the helper message entity
                    // then attach a flashing component to it
                    if let Some(helper_message) = data.world.exec(|ui_finder: UiFinder<'_>| {
                        ui_finder.find("disclaimer_helper")
                    }) {
                        // get the UiText color
                        let uitext_storage = data.world.read_storage::<UiText>();
                        let text_color = uitext_storage.get(helper_message).unwrap().color;
                        
                        // add flashing component to the text
                        let mut flashing_comp_write_storage = data.world.write_storage::<FlashingComp>();
                        let _insert_result = flashing_comp_write_storage.insert(helper_message, FlashingComp::new(text_color));
                    }
                    println!("======= Switch State      =======");
                    return Trans::Switch(Box::new(DisclaimerState::default()));
                }
            }
        } 
        
        Trans::None
    }

}