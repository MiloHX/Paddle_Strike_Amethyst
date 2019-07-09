//================
// Import modules
//================

// amethyst modules
use amethyst::{
    ecs::Entity,
    prelude::*,
    assets::{
        Completion, 
        ProgressCounter,
        Loader,
    },
    ui::{
        UiText,
        UiImage,
        UiTransform,
        Anchor,
        TtfFormat,
        Stretch,
    },
};

// local modules
use crate::components::FlashingComp;
use crate::components::FlashingStyle;
use crate::states::disclaimer_state::DisclaimerState;
use crate::states::state_event::CustomStateEvent;

//=======================
// Declare loading state
//=======================
//
// Note that if it is not a unit struct (with no fields)
// you cannot directly use it as the parameter of the Application::new() function
// a seperate method (here we use default())to return an instance (Self) need to be used
#[derive(Default)]
pub struct LoadingState {
    // Loading screen entity
    loading_screen:     Option<Entity>,
    // Tracks loaded assets.
    loading_progress:   Option<ProgressCounter>,
    // Temp
    delay_frame_count:  u32,
}

//=======================
// Implement State trait
//=======================
impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for LoadingState {

    //----------------
    // Start up tasks
    //----------------
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // init loading progress
        self.loading_progress = Some(ProgressCounter::new());

        // load font for loading screen
        let font = data.world.read_resource::<Loader>().load(
            "assets/fonts/players.ttf",
            TtfFormat,
            (),
            &data.world.read_resource(),
        );

        // set the transform of the loading screen
        let loading_transform = UiTransform::new(
            "loading".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            0.,
            1.,
            300.,
            50.,)
            .with_stretch(Stretch::XY {
                x_margin: 0., 
                y_margin: 0., 
                keep_aspect_ratio: false
            });

        // set loading text color
        let loading_color = [1., 1., 0., 1.];

        // save the loading screen entity
        self.loading_screen = Some(data.world
            .create_entity()
            .with(loading_transform)
            .with(UiImage::SolidColor([1., 1., 1., 1.,]))
            .with(UiText::new(
                font,
                "Loading".to_string(),
                loading_color,
                56.,
            ))
            .with(FlashingComp::new(
                loading_color, 
                true, 
                1., 
                0.8, 
                FlashingStyle::Darkening, [1., 1., 0., 0.]),
            )
            .build());

            // LOAD SOMETHING HERE!
    }

    //---------------
    // Stoping tasks 
    //---------------
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // clean up
        if let Some(screen) = self.loading_screen {
            let _result = data.world.entities().delete(screen);
        }
        self.loading_screen = None;
        self.loading_progress = None;
        self.delay_frame_count = 0;
    }

    //--------------
    // Update tasks 
    //--------------
    //
    // This will be called repeatly until transition to other state
    fn update(&mut self, data: StateData<'_, GameData<'a, 'a>>)-> Trans<GameData<'a, 'a>, CustomStateEvent> {

        // update game data
        data.data.update(&data.world);

        // here will get the counter as a reference
        if let Some(ref counter) = self.loading_progress.as_ref() {
            match counter.complete() {
                Completion::Loading  => {
                    // loading onging
                }
                Completion::Failed   => {
                    info!("======= Loading Failed    =======");
                }
                Completion::Complete => {
                    // TESTING ONLY!
                    self.delay_frame_count += 1;
                    if self.delay_frame_count < 150  {
                        return Trans::None;
                    }
                    info!("======= Loading Completed =======");
                    info!("======= Switch State      =======");
                    return Trans::Switch(Box::new(DisclaimerState::default()));
                }
            }
        } 
        
        Trans::None
    }

}