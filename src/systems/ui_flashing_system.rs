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
    // define what data to be retreived from the storage
    type SystemData = (
        Read<'s, Time>, 
        WriteStorage<'s, UiText>,
        ReadStorage<'s, FlashingComp>,
    );

    // system execution (run every frame)
    fn run(&mut self, (sys_time, mut texts, flashing_items): Self::SystemData) {
        //---------------------
        // flashing the UiText 
        //---------------------
        for (text, flashing_item,) in (&mut texts, &flashing_items,).join() {
            // flashing is enabled
            if flashing_item.is_flashing {
                // calculate the flashing factor based on the rate and intensity setting
                let factor: f32 = (
                    sys_time.absolute_real_time_seconds() as f32 * 5. * flashing_item.rate
                ).sin() * flashing_item.intensity;
                // update text color
                text.color = [
                    (flashing_item.orginal_color[0] + factor).min(1.).max(0.),  // R
                    (flashing_item.orginal_color[1] + factor).min(1.).max(0.),  // G
                    (flashing_item.orginal_color[2] + factor).min(1.).max(0.),  // B
                     flashing_item.orginal_color[3],                            // A
                ];

            // flashing is disabled
            } else {
                // Use original color 
                text.color = [
                    flashing_item.orginal_color[0], 
                    flashing_item.orginal_color[1], 
                    flashing_item.orginal_color[2], 
                    flashing_item.orginal_color[3],
                ];               
            }
        }
    }
}