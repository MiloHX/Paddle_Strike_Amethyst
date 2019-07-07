//=========================
// Import amethyst modules
//=========================
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage, ReadStorage},
    ui::UiText,
};

//======================
// Import local modules
//======================
use crate::components::FlashingComp;

//==============================
// Declare Text Flashing System
//==============================
pub struct UiFlashingSystem;

//========================
// Implement System trait
//========================
impl<'s> System<'s> for UiFlashingSystem {
    type SystemData = (
        Read<'s, Time>, 
        WriteStorage<'s, UiText>,
        ReadStorage<'s, FlashingComp>,
    );

    fn run(&mut self, (sys_time, mut texts, falshing_items): Self::SystemData) {
        let factor: f32 = ((sys_time.absolute_real_time_seconds() * 5.).sin() * 0.4) as f32;
        for (text, falshing_item,) in (&mut texts, &falshing_items,).join() {
            text.color = [
                (falshing_item.orginal_color[0] + factor).min(1.).max(0.),
                (falshing_item.orginal_color[1] + factor).min(1.).max(0.),
                (falshing_item.orginal_color[2] + factor).min(1.).max(0.),
                (falshing_item.orginal_color[3] + factor).min(1.).max(0.),
            ];
        }
    }
}