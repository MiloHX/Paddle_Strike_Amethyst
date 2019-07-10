use amethyst::{
    prelude::*,
    ui::{
        UiFinder,
        UiText,
    },
};

// local modules
use crate::components::flashing_comp::FlashingComp;
use crate::components::flashing_comp::FlashingStyle;

pub fn impl_flashing_comp(
    ui_text_id:     &str, 
    data:           &mut StateData<GameData>,
    is_flashing:    bool,
    rate:           f32, 
    intensity:      f32,
    style:          FlashingStyle,
    rgba_factors:   [f32; 4]
) {
    info!("Try to find this id: {}", ui_text_id);
    if let Some(loading_text) = data.world.exec(|ui_finder: UiFinder<'_>| {
        ui_finder.find(ui_text_id)
    }) {
        info!("This ID has been found: {}", ui_text_id);
        // get the UiText color
        let ui_text_storage = data.world.read_storage::<UiText>();
        let text_color = ui_text_storage.get(loading_text).unwrap().color;

        // add flashing component to the text
        let mut flashing_comp_write_storage = data.world.write_storage::<FlashingComp>();
        let _insert_result = flashing_comp_write_storage.insert(
            loading_text, 
            FlashingComp::new(text_color, is_flashing, rate, intensity, style, rgba_factors),
        );
    }
}