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
                let factor: f32 = (
                    sys_time.absolute_real_time_seconds() as f32 
                    * 4. 
                    * jumping_item.rate
                    - (0.1 * jumping_item.order as f32)
                ).sin() * 0.5 * 75. * jumping_item.height;
                tran.local_y = jumping_item.orginal_pos.1 + factor.max(0.);
             } else {
                tran.local_y = jumping_item.orginal_pos.1;
             }

        }
    }
}