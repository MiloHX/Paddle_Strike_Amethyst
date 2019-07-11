use amethyst::{
    prelude::*,
    ui::{
        UiFinder,
        UiText,
        UiTransform,
    },
};

// local modules
use crate::components::ui_flashing_comp::UiFlashingComp;
use crate::components::ui_flashing_comp::UiFlashingStyle;
use crate::components::ui_swinging_comp::UiSwingingComp;
use crate::components::ui_swinging_comp::UiSwingingStyle;

pub fn impl_flashing_comp (
    ui_text_id:     &str, 
    data:           &mut StateData<GameData>,
    is_flashing:    bool,
    rate:           f32, 
    intensity:      f32,
    style:          UiFlashingStyle,
    rgba_factors:   [f32; 4],
) {
    if let Some(text_entity) = data.world.exec(|ui_finder: UiFinder<'_>| {
        ui_finder.find(ui_text_id)
    }) {
        // get the UiText color
        let ui_text_storage = data.world.read_storage::<UiText>();
        let text_color = ui_text_storage.get(text_entity).unwrap().color;

        // add flashing component to the entity
        let mut flashing_comp_write_storage = data.world.write_storage::<UiFlashingComp>();
        let _insert_result = flashing_comp_write_storage.insert(
            text_entity, 
            UiFlashingComp::new(text_color, is_flashing, rate, intensity, style, rgba_factors),
        );
    }
}

pub fn impl_swinging_comp (
    ui_item_id: &str,
    data:       &mut StateData<GameData>,
    is_swinging:bool,
    rate:       f32,
    amplitude:  f32,
    style:      UiSwingingStyle,
) {
    if let Some(ui_entity) = data.world.exec(|ui_finder: UiFinder<'_>| {
        ui_finder.find(ui_item_id) 
    }) {
        // get the original x and y values 
        let ui_tran_storage = data.world.read_storage::<UiTransform>();
        let ui_item = ui_tran_storage.get(ui_entity).unwrap();
        let (org_x, org_y) = (ui_item.local_x, ui_item.local_y);

        // add swinging component to the entity
        let mut swinging_comp_write_storage = data.world.write_storage::<UiSwingingComp>();
        let _insert_result = swinging_comp_write_storage.insert(
            ui_entity, 
            UiSwingingComp::new((org_x, org_y), is_swinging, rate, amplitude, style),
        );
    }
}