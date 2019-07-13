// amethyst modules
use amethyst::{
    ecs::prelude::{Join, System, WriteStorage, ReadStorage,},
    ui::UiTransform,
};

// local modules
use crate::components::ui_cursor_comp::UiCursorComp;
use crate::components::ui_cursor_option_comp::UiCursorOptionComp;
use crate::components::ui_cursor_option_comp::UiCursorOptionStyle;
use crate::components::ui_glowing_comp::UiGlowingComp;

//========================
// Cursor Position System
//========================
pub struct UiCursorSystem;

impl<'s> System<'s> for UiCursorSystem {
    // define what data to be retreived from the storage
    type SystemData = (
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, UiCursorComp>,
        ReadStorage<'s, UiCursorOptionComp>,
        WriteStorage<'s, UiGlowingComp>
    );

    fn run(&mut self, (mut trans, cursors, options, mut glowings): Self::SystemData) {
        for (tran, cursor,) in (&mut trans, &cursors,).join() {
            if cursor.pos_list[cursor.current_pos].1 != tran.local_y {
                // move cursor
                tran.local_y = cursor.pos_list[cursor.current_pos].1;
                // highlight option
                for (option, glowing) in (&options, &mut glowings).join() {
                    if option.group == cursor.group {
                        if option.id == cursor.pos_id_list[cursor.current_pos] {
                            match option.style {
                                UiCursorOptionStyle::Glowing => {
                                    glowing.is_glowing = true;
                                }
                            }
                        } else {
                            match option.style {
                                UiCursorOptionStyle::Glowing => {
                                    glowing.is_glowing = false;
                                }
                            }   
                        }
                    }
                }
            }
        }
    }
}