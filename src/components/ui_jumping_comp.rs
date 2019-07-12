// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Clone, new)]
pub struct UiJumpingComp {
    pub orginal_pos:    (f32, f32),     // saved original position
    pub group:          String,
    pub order:          usize, 
    pub is_jumping:     bool,
    pub rate:           f32,   
    pub height:         f32,   
}

// make it component
impl Component for UiJumpingComp {
    type Storage = DenseVecStorage<Self>;
}