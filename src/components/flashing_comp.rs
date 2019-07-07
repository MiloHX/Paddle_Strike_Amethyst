use amethyst::{
    ecs::{Component, VecStorage},
};

#[derive(Clone, Debug)]
pub struct FlashingComp {
    pub orginal_color: [f32; 4],
    pub is_flashing: bool,
}

// impl FlashingComp {
//     fn new(text_color: [f32; 4]) -> Self {
//         FlashingComp {
//             orginal_color: text_color,
//             is_flashing: true,
//         }
//     }
// }

impl Default for FlashingComp {
    fn default() -> Self {
        FlashingComp {
            orginal_color: [0.6, 0.6, 0.6, 0.6,],
            is_flashing: true,
        }
    }
}

impl Component for FlashingComp {
    type Storage = VecStorage<Self>;
}