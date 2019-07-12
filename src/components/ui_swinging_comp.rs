// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

//================
// Swinging Style
//================
#[derive(Clone)]
#[allow(dead_code)]
pub enum UiSwingingStyle {
    Horizontal,     // ui element swing horizontally
    Vertical,       // ui element swing vertically
}

#[derive(Clone, new)]
pub struct UiSwingingComp {
    pub orginal_pos:    (f32, f32),     // saved original position
    pub is_swinging:    bool,           // is it swinging?
    pub rate:           f32,            // swinging rate (default is 1.0)
    pub amplitude:      f32,            // swinging amplitude
    pub style:          UiSwingingStyle,// flashing style
}

// make it component
impl Component for UiSwingingComp {
    type Storage = DenseVecStorage<Self>;
}