//================
// Import modules
//================

// amethyst modules
use amethyst::{
    ecs::Entity,
    assets::{
        ProgressCounter,
    },
    prelude::*,
    ui::{
        UiCreator,
        UiFinder,
        UiText,
    },
};

// local modules
use crate::components::FlashingComp;
use crate::components::FlashingStyle;
use crate::states::state_event::CustomStateEvent;

//=========================
// Define disclaimer state
//=========================
#[derive(Default)]
pub struct DisclaimerState {
    // Loading screen entity
    disclaimer_screen:  Option<Entity>,
    // Tracks loaded assets.
    loading_progress:   Option<ProgressCounter>,
    // Temp
    delay_frame_count:  u32,
}

impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for DisclaimerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // init loading progress
        self.loading_progress = Some(ProgressCounter::new());
        // initialize disclaimer with a ron file.
        // here we need to handle Some and None case of the loading_progress
        let mut screen:Option<Entity> = None;
        if let Some(counter) = &mut self.loading_progress {
            // Make use of UiCreator to create the UI defined in disclaimer.ron
            data.world.exec(|mut creator: UiCreator<'_>| {
                screen = Some(creator.create("ui/disclaimer.ron", counter));
            });
        }
        self.disclaimer_screen = screen;
        // use UiFinder to get the helper message entity
        // then attach a flashing component to it
        if let Some(helper_message) = data.world.exec(|ui_finder: UiFinder<'_>| {
            ui_finder.find("disclaimer_helper")
        }) {
            info!("FLASHING COMP ADDED");
            // get the UiText color
            let uitext_storage = data.world.read_storage::<UiText>();
            let text_color = uitext_storage.get(helper_message).unwrap().color;
            
            // add flashing component to the helper text
            let mut flashing_comp_write_storage = data.world.write_storage::<FlashingComp>();
            let _insert_result = flashing_comp_write_storage.insert(
                helper_message, 
                FlashingComp::new(text_color, true, 1., 0.8, FlashingStyle::Darkening, [1., 1., 0., 0.]),
            );
        }
    }

    fn update(&mut self, data: StateData<'_, GameData<'a, 'a>>)-> Trans<GameData<'a, 'a>, CustomStateEvent> {
        data.data.update(&data.world);
        self.delay_frame_count += 1;
        if self.delay_frame_count == 150  {
            if let Some(screen_entity) = self.disclaimer_screen {
                let _result = data.world.delete_entity(screen_entity);
                info!("Screen removed!");
            }
        }
        Trans::None
    }

}