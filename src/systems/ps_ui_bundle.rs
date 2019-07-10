use amethyst::{
    core::bundle::SystemBundle, 
    ecs::prelude::DispatcherBuilder, 
    error::Error,
};
use crate::systems::ui_flashing_system::UiFlashingSystem;

pub struct PsUiBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PsUiBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(UiFlashingSystem, "ui_flashing_system", &[]);
        Ok(())
    }
}