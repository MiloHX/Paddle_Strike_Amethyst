// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::mx_utils::mx_timer::MxTimer;

#[derive(Clone)]
pub struct UiWavingComp {
    pub orginal_pos:    (f32, f32),     // saved original position
    pub group:          String,
    pub order:          usize,
    pub is_waving:      bool,
    pub rate:           f32,
    pub height:         f32,
    pub low_cut:        f32,
    pub high_cut:       f32,
    pub delay:          f32,
    pub timer:          MxTimer,
    pub play_time:      f32,
    pub wait_time:      f32,
    pub playing:        bool,

}

impl UiWavingComp {
    pub fn new(
        orginal_pos:    (f32, f32),
        group:          String,
        order:          usize,
        is_waving:      bool,
        rate:           f32,
        height:         f32,
        low_cut:        f32,
        high_cut:       f32,
        delay:          f32,
        play_time:      f32,
        wait_time:      f32,
    ) -> Self {
        UiWavingComp {
            orginal_pos,
            group,
            order,
            is_waving,
            rate,
            height, 
            low_cut, 
            high_cut,
            delay,
            timer:      MxTimer::new(play_time, false), 
            play_time,
            wait_time,
            playing:    true,
        }
    }
}

// make it component
impl Component for UiWavingComp {
    type Storage = DenseVecStorage<Self>;
}