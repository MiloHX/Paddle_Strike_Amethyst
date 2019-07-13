// amethyst modules
use amethyst::{
    ecs::{Entity, Component, DenseVecStorage},
};

//=====================
// Cursor Option Style
//=====================
#[derive(Clone)]
#[allow(dead_code)]
pub enum UiCursorOptionStyle {
    Glowing,
}

impl Default for UiCursorOptionStyle{
    fn default() -> Self {
        UiCursorOptionStyle::Glowing
    }
}

//===============
// Cursor Option 
//===============
#[derive(Clone, new)]
pub struct UiCursorOptionComp {
    pub group:      String,
    pub id:         String,
    pub style:      UiCursorOptionStyle,
    pub entity:     Entity,
}

// make it component
impl Component for UiCursorOptionComp {
    type Storage = DenseVecStorage<Self>;
}