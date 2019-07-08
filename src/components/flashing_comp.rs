use amethyst::{
    ecs::{Component, VecStorage},
};

#[derive(Clone, Debug)]
pub struct FlashingComp {
    pub orginal_color:  [f32; 4],   // Saved original color of the text
    pub is_flashing:    bool,       // control if the text is flashing or not
    pub rate:           f32,        // flashing rate (default is 1.0)
    pub intensity:      f32,        // flashing intensity (from 0.0 to 1.0, default is 0.4)
}

impl FlashingComp {
    pub fn new(text_color: [f32; 4]) -> Self {
        FlashingComp {
            orginal_color:  text_color,
            is_flashing:    true,
            rate:           1.,
            intensity:      0.4,
        }
    }
}

impl Default for FlashingComp {
    fn default() -> Self {
        FlashingComp {
            orginal_color:  [0.6, 0.6, 0.6, 0.6,],
            is_flashing:    true,
            rate:           1.,
            intensity:      0.4,
        }
    }
}

impl Component for FlashingComp {
    type Storage = VecStorage<Self>;
}