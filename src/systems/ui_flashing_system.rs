use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Read, 
        System, 
        WriteStorage, 
        ReadStorage, 
        Join,
    },
    ui::UiText,
};

// local modules
use crate::components::ui_flashing_comp::UiFlashingComp;

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
        //--------------------------
        // flashing some ui element 
        //--------------------------
        for (text, flashing_item,) in (&mut texts, &flashing_items,).join() {
            if flashing_item.is_hiding {
                // Use original color 
                text.color = [
                    flashing_item.orginal_color[0], 
                    flashing_item.orginal_color[1], 
                    flashing_item.orginal_color[2], 
                    0.,
                ];     
            } else if flashing_item.is_flashing {
                // calculate the flashing factor based on the rate
                let should_be_on = (
                            sys_time.absolute_real_time_seconds() as f32 * 25. * flashing_item.rate
                        ).sin() >= 0.;
                if should_be_on {
                    text.color = [
                        flashing_item.flashing_color[0], 
                        flashing_item.flashing_color[1], 
                        flashing_item.flashing_color[2], 
                        flashing_item.flashing_color[3],
                    ];  
                } else {
                    text.color = [
                        flashing_item.orginal_color[0], 
                        flashing_item.orginal_color[1], 
                        flashing_item.orginal_color[2], 
                        0.,
                    ];    
                   
                }              
            } else {
                // reset to original color
                if flashing_item.reset_color {
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
}