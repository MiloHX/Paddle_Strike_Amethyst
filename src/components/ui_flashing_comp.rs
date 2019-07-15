// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Clone, new)]
pub struct UiFlashingComp {
    pub orginal_color:  [f32; 4],       // Saved original color
    pub is_flashing:    bool,           // is it flashinging?
    pub is_hiding:      bool,           // is it hiding?
    pub rate:           f32,            // flashing rate (default is 1.0)
}

// make it component
impl Component for UiFlashingComp {
    type Storage = DenseVecStorage<Self>;
}
