// amethyst modules
use amethyst::{
    ecs::prelude::{Join, System, WriteStorage, ReadStorage,},
    ui::UiTransform,
};

// local modules
use crate::components::ui_cursor_movement_comp::UiCursorMovementComp;

//========================
// Cursor Position System
//========================
pub struct UiMainMenuCursorPosSystem;

impl<'s> System<'s> for UiMainMenuCursorPosSystem {
    // define what data to be retreived from the storage
    type SystemData = (
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, UiCursorMovementComp>,
    );

    fn run(&mut self, (mut trans, cursor_movements): Self::SystemData) {
        for (tran, cursor_move,) in (&mut trans, &cursor_movements,).join() {
            if cursor_move.current_pos.get_local_y() != tran.local_y {
                tran.local_y = cursor_move.current_pos.get_local_y();
            }
        }
    }
}