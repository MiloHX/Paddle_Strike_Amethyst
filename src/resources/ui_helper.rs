use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::{
        UiText,
        UiTransform,
        UiFinder,
    },
};

// local modules
use crate::components::ui_glowing_comp::UiGlowingComp;
use crate::components::ui_glowing_comp::UiGlowingStyle;
use crate::components::ui_swinging_comp::UiSwingingComp;
use crate::components::ui_swinging_comp::UiSwingingStyle;
use crate::components::ui_waving_comp::UiWavingComp;
use crate::components::ui_cursor_comp::UiCursorComp;
use crate::components::ui_cursor_option_comp::UiCursorOptionComp;
use crate::components::ui_cursor_option_comp::UiCursorOptionStyle;

pub fn impl_glowing_comp (
    text_entity:    &Entity, 
    data:           &mut StateData<GameData>,
    is_glowing:    bool,
    rate:           f32, 
    intensity:      f32,
    style:          UiGlowingStyle,
    rgba_factors:   [f32; 4],
) {
    // get the UiText color
    let text_color = get_text_color(text_entity, data);

    // add glowing component to the entity
    let mut glowing_comp_write_storage = data.world.write_storage::<UiGlowingComp>();
    let _insert_result = glowing_comp_write_storage.insert(
        *text_entity, 
        UiGlowingComp::new(text_color, is_glowing, rate, intensity, style, rgba_factors),
    );
}

pub fn get_text_color (
    text_entity:    &Entity,
    data:           &mut StateData<GameData>,
) -> [f32; 4]  {
    // get the UiText color
    let ui_text_storage = data.world.read_storage::<UiText>();
    ui_text_storage.get(*text_entity).unwrap().color
}

pub fn impl_swinging_comp (
    ui_entity:  &Entity,
    data:       &mut StateData<GameData>,
    is_swinging:bool,
    rate:       f32,
    amplitude:  f32,
    style:      UiSwingingStyle,
) {
    // get the original x and y values 
    let ui_tran_storage = data.world.read_storage::<UiTransform>();
    let ui_item = ui_tran_storage.get(*ui_entity).unwrap();
    let (org_x, org_y) = (ui_item.local_x, ui_item.local_y);

    // add swinging component to the entity
    let mut swinging_comp_write_storage = data.world.write_storage::<UiSwingingComp>();
    let _insert_result = swinging_comp_write_storage.insert(
        *ui_entity, 
        UiSwingingComp::new((org_x, org_y), is_swinging, rate, amplitude, style),
    );
}

pub fn impl_waving_comp (
    ui_entity:  &Entity,
    data:       &mut StateData<GameData>,
    group:      &str,
    order:      usize,
    is_waving: bool,
    rate:       f32,
    height:     f32,
    low_cut:    f32,
    high_cut:   f32,
    delay:      f32,
    play_time:  f32,
    wait_time:  f32
) {
    // get the original x and y values 
    let ui_tran_storage = data.world.read_storage::<UiTransform>();
    let ui_item = ui_tran_storage.get(*ui_entity).unwrap();
    let (org_x, org_y) = (ui_item.local_x, ui_item.local_y);

    // add swinging component to the entity
    let mut swinging_comp_write_storage = data.world.write_storage::<UiWavingComp>();
    let _insert_result = swinging_comp_write_storage.insert(
        *ui_entity, 
        UiWavingComp::new(
            (org_x, org_y), 
            group.to_string(), 
            order, 
            is_waving, 
            rate, 
            height, 
            low_cut, 
            high_cut,
            delay,
            play_time,
            wait_time),
    );
}

pub fn impl_cursor_option_comp (
    group:          &str,
    id:             &str,
    ui_entity:      &Entity,
    data:           &mut StateData<GameData>, 
    style:          UiCursorOptionStyle,
) {
    let mut cursor_option_write_storage = data.world.write_storage::<UiCursorOptionComp>();
    let _insert_result = cursor_option_write_storage.insert(
        *ui_entity, 
        UiCursorOptionComp::new(
            group.to_string(), 
            id.to_string(), 
            style, 
            *ui_entity
        ),
    );
}

pub fn impl_cursor_comp (
    ui_entity:      &Entity,
    data:           &mut StateData<GameData>,
    group:          &str,
    pos_list:       Vec<(f32, f32)>,
    pos_id_list:    Vec<String>,
) {
    // add cursor movement component to the entity
    let mut cursor_write_storage = data.world.write_storage::<UiCursorComp>();
    let _insert_result = cursor_write_storage.insert(
        *ui_entity, 
        UiCursorComp::new(
            group.to_string(),
            0, 
            pos_list, 
            pos_id_list
        ),
    );
}

pub fn move_cursor (
    cursor:     &Entity,
    data:       &mut StateData<GameData>,
    direction:  bool,
) {
    let mut cursor_storage = data.world.write_storage::<UiCursorComp>();
    if let Some(cursor) = cursor_storage.get_mut(*cursor) {
        cursor.advance_pos(direction);
    } 
}

pub fn get_cursor_action (
    cursor:     &Entity,
    data:       &mut StateData<GameData>,
) -> String {
    let mut cursor_storage = data.world.write_storage::<UiCursorComp>();
    if let Some(cursor) = cursor_storage.get_mut(*cursor) {
        return cursor.pos_id_list[cursor.current_pos].clone();
    } 
    "".to_string()
}

pub fn impl_bulk_waving (
    item_ids:   Vec<&str>,
    data:       &mut StateData<GameData>,
    group:      &str,
    is_waving: bool,
    rate:       f32,
    height:     f32,
    low_cut:    f32,
    high_cut:   f32,
    delay:      f32,
    play_time:  f32,
    wait_time:  f32,
) {
    let mut order:usize = 0;
    for item_id in item_ids {
        if let Some(item) = data.world.exec(|ui_finder: UiFinder<'_>| {
            ui_finder.find(item_id)
        })  {
            impl_waving_comp(
                &item,
                data,
                group,
                order,
                is_waving,
                rate,
                height,
                low_cut,
                high_cut,
                delay,
                play_time,
                wait_time,
            );
            order += 1;
        }
    }
}