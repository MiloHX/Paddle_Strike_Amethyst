use amethyst::{
    core::bundle::SystemBundle, 
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};
use crate::systems::ui_glowing_system::UiGlowingSystem;
use crate::systems::ui_swinging_system::UiSwingingSystem;
use crate::systems::ui_cursor_system::UiCursorSystem;
use crate::systems::ui_waving_system::UiWavingSystem;
use crate::systems::ui_flashing_system::UiFlashingSystem;

pub struct PsUiBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PsUiBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(UiGlowingSystem, "ui_glowing_system", &[]);
        builder.add(UiSwingingSystem, "ui_swinging_system", &[]);
        builder.add(UiCursorSystem, "ui_cursor_system", &[]);
        builder.add(UiWavingSystem, "ui_waving_system", &[]);
        builder.add(UiFlashingSystem, "ui_flashing_system", &["ui_glowing_system"]);
        Ok(())
    }
}