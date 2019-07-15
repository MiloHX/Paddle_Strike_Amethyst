
use std::f32::consts::PI;
use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Read, 
        System, 
        WriteStorage,  
        Join,
    },
    ui::UiTransform,
};

use crate::components::ui_waving_comp::UiWavingComp;

pub struct UiWavingSystem;

impl<'s> System<'s> for UiWavingSystem {
    type SystemData = (
        Read<'s, Time>, 
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiWavingComp>,
    );

    fn run(&mut self, (sys_time, mut transforms, mut waving_items): Self::SystemData) {
        //------------------------------
        // make some ui elements waving
        //------------------------------
        let mut next_cycle = false;
        for (tran, waving_item,) in (&mut transforms, &mut waving_items,).join() {
            if waving_item.is_waving {

                if !waving_item.timer.is_running() || next_cycle {
                    if waving_item.order > 0 {
                        waving_item.timer.set(waving_item.order as f32 * waving_item.delay, false);
                        waving_item.playing = false;                       
                    } 
                    waving_item.timer.start();    

                } else {
                    let finshed = waving_item.timer.update(&sys_time);

                    if waving_item.playing {
                        let sin_value = 
                            (waving_item.timer.get_ratio() * PI * 2.).sin()
                            .max(waving_item.low_cut).min(waving_item.high_cut);
                        let factor   = sin_value * 20. * waving_item.height;
                        tran.local_y = waving_item.orginal_pos.1 + factor;
                    } else {
                        tran.local_y = waving_item.orginal_pos.1;
                    }

                    if finshed {
                        if waving_item.playing {
                            waving_item.timer.set(waving_item.wait_time, false);
                            waving_item.timer.start();
                            waving_item.playing = false;
                        } else {
                            waving_item.timer.set(waving_item.play_time, false);
                            waving_item.timer.start();
                            waving_item.playing = true;
                            if waving_item.order == 0 {
                                next_cycle = true;
                            }
                        }
                    }
                }
            }
        }
    }
}