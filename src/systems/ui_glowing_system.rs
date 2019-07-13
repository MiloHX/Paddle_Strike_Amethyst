// amethyst modules
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage, ReadStorage,},
    ui::UiText,
};

// local modules
use crate::components::ui_glowing_comp::UiGlowingComp;
use crate::components::ui_glowing_comp::UiGlowingStyle;

//======================
// Text Glowing System
//======================
pub struct UiGlowingSystem;

impl<'s> System<'s> for UiGlowingSystem {
    // define what data to be retreived from the storage
    type SystemData = (
        Read<'s, Time>, 
        WriteStorage<'s, UiText>,
        ReadStorage<'s, UiGlowingComp>,
    );

    // system execution (run every frame)
    fn run(&mut self, (sys_time, mut texts, glowing_items): Self::SystemData) {
        //---------------------
        // glowing the UiText 
        //---------------------
        for (text, glowing_item,) in (&mut texts, &glowing_items,).join() {
            // glowing is enabled
            if glowing_item.is_glowing {
                let factor: f32;
                // calculate the glowing factor based on the rate, intensity and style setting
                match glowing_item.style {
                    UiGlowingStyle::TwoWays => {
                        factor = (
                            sys_time.absolute_real_time_seconds() as f32 * 5. * glowing_item.rate
                        ).sin() * 0.5 * glowing_item.intensity;
                    }
                    UiGlowingStyle::Lightening => {
                        factor = (
                            (sys_time.absolute_real_time_seconds() as f32 * 5. * glowing_item.rate
                        ).sin() + 1.) * 0.5 * glowing_item.intensity;                        
                    }
                    UiGlowingStyle::Darkening => {
                        factor = (
                            (sys_time.absolute_real_time_seconds() as f32 * 5. * glowing_item.rate
                        ).sin() - 1.) * 0.5 * glowing_item.intensity;                        
                    }
                }
                // update text color
                text.color = [
                    (glowing_item.orginal_color[0] + factor * glowing_item.rgba_factors[0]).min(1.).max(0.),  // R
                    (glowing_item.orginal_color[1] + factor * glowing_item.rgba_factors[1]).min(1.).max(0.),  // G
                    (glowing_item.orginal_color[2] + factor * glowing_item.rgba_factors[2]).min(1.).max(0.),  // B
                    (glowing_item.orginal_color[3] + factor * glowing_item.rgba_factors[3]).min(1.).max(0.),  // A
                ];
            
            // glowing is disabled
            } else {
                // Use original color 
                text.color = [
                    glowing_item.orginal_color[0], 
                    glowing_item.orginal_color[1], 
                    glowing_item.orginal_color[2], 
                    glowing_item.orginal_color[3],
                ];               
            }
        }
    }
}