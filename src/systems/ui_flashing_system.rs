// amethyst modules
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage, ReadStorage,},
    ui::UiText,
};

// local modules
use crate::components::ui_flashing_comp::UiFlashingComp;
use crate::components::ui_flashing_comp::UiFlashingStyle;

//======================
// Text Flashing System
//======================
pub struct UiFlashingSystem;

impl<'s> System<'s> for UiFlashingSystem {
    // define what data to be retreived from the storage
    type SystemData = (
        Read<'s, Time>, 
        WriteStorage<'s, UiText>,
        ReadStorage<'s, UiFlashingComp>,
    );

    // system execution (run every frame)
    fn run(&mut self, (sys_time, mut texts, flashing_items): Self::SystemData) {
        //---------------------
        // flashing the UiText 
        //---------------------
        for (text, flashing_item,) in (&mut texts, &flashing_items,).join() {
            // flashing is enabled
            if flashing_item.is_flashing {
                let factor: f32;
                // calculate the flashing factor based on the rate, intensity and style setting
                match flashing_item.style {
                    UiFlashingStyle::TwoWays => {
                        factor = (
                            sys_time.absolute_real_time_seconds() as f32 * 5. * flashing_item.rate
                        ).sin() * 0.5 * flashing_item.intensity;
                    }
                    UiFlashingStyle::Lightening => {
                        factor = (
                            (sys_time.absolute_real_time_seconds() as f32 * 5. * flashing_item.rate
                        ).sin() + 1.) * 0.5 * flashing_item.intensity;                        
                    }
                    UiFlashingStyle::Darkening => {
                        factor = (
                            (sys_time.absolute_real_time_seconds() as f32 * 5. * flashing_item.rate
                        ).sin() - 1.) * 0.5 * flashing_item.intensity;                        
                    }
                }
                // update text color
                text.color = [
                    (flashing_item.orginal_color[0] + factor * flashing_item.rgba_factors[0]).min(1.).max(0.),  // R
                    (flashing_item.orginal_color[1] + factor * flashing_item.rgba_factors[1]).min(1.).max(0.),  // G
                    (flashing_item.orginal_color[2] + factor * flashing_item.rgba_factors[2]).min(1.).max(0.),  // B
                    (flashing_item.orginal_color[3] + factor * flashing_item.rgba_factors[3]).min(1.).max(0.),  // A
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