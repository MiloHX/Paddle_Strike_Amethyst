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

use crate::components::ui_jumping_comp::UiJumpingComp;

pub struct UiJumpingSystem;

impl<'s> System<'s> for UiJumpingSystem {
    type SystemData = (
        Read<'s, Time>, 
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, UiJumpingComp>,
    );

    fn run(&mut self, (sys_time, mut transforms, jumping_items): Self::SystemData) {
        //----------------------------
        // make some ui elements jump
        //----------------------------
        for (tran, jumping_item,) in (&mut transforms, &jumping_items,).join() {
             if jumping_item.is_jumping {
                let sin_val: f32 = (
                    sys_time.absolute_real_time_seconds() as f32 
                    * 4. 
                    * jumping_item.rate
                    - (jumping_item.delay * jumping_item.order as f32)
                ).sin();
                let sin_val_cut = (sin_val - jumping_item.cut_off).max(0.);
                let factor = sin_val_cut * 0.5 * 75. * jumping_item.height;
                tran.local_y = jumping_item.orginal_pos.1 + factor;
             } else {
                tran.local_y = jumping_item.orginal_pos.1;
             }

        }
    }
}