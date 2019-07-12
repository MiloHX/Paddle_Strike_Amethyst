use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Read, 
        System, 
        WriteStorage, 
        ReadStorage, 
        Join,
    },
    ui::UiTransform,
};

// local modules
use crate::components::ui_swinging_comp::UiSwingingComp;
use crate::components::ui_swinging_comp::UiSwingingStyle;

pub struct UiSwingingSystem;

impl<'s> System<'s> for UiSwingingSystem {
    // define what data to be retreived from the storage
    type SystemData = (
        Read<'s, Time>, 
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, UiSwingingComp>,
    );

    // system execution (run every frame)
    fn run(&mut self, (sys_time, mut transforms, swinging_items): Self::SystemData) {
        //--------------------------
        // swinging some ui element 
        //--------------------------
        for (tran, swinging_item) in (&mut transforms, &swinging_items,).join() {
            if swinging_item.is_swinging {
                // calculate the flashing factor based on the rate, amplitude
                let factor: f32 = (
                    sys_time.absolute_real_time_seconds() as f32 * 5. * swinging_item.rate
                ).sin() * 0.5 * 6. * swinging_item.amplitude;
                // apply swinging
                match swinging_item.style {
                    UiSwingingStyle::Horizontal => {
                        tran.local_x = swinging_item.orginal_pos.0 + factor;
                    }
                    UiSwingingStyle::Vertical => {
                        tran.local_x = swinging_item.orginal_pos.1 + factor;
                    }
                }
            }
        }
    }
}