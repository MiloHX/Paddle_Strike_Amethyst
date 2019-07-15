// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

//================
// Glowing Style
//================
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum UiGlowingStyle {
    TwoWays,        // color get lightened and darkened
    Lightening,     // color only get lightened
    Darkening,      // color only get darkened
}

//====================
// Glowing Component
//====================
#[derive(Clone, new)]
pub struct UiGlowingComp {
    pub orginal_color:  [f32; 4],       // Saved original color
    pub is_glowing:    bool,            // control if the ui element is glowing or not
    pub rate:           f32,            // glowing rate (default is 1.0)
    pub intensity:      f32,            // glowing intensity (from 0.0 to 2.0, default is 0.4)
    pub style:          UiGlowingStyle, // glowing style
    pub rgba_factors:   [f32; 4],       // rgba channel factors
}

// default values
impl Default for UiGlowingComp {
    fn default() -> Self {
        UiGlowingComp {
            orginal_color:  [0.6, 0.6, 0.6, 0.6,],
            is_glowing:    true,
            rate:           1.,
            intensity:      0.4,
            style:          UiGlowingStyle::TwoWays,
            rgba_factors:   [1., 1., 1., 0.,],
        }
    }
}

// make it component
impl Component for UiGlowingComp {
    type Storage = DenseVecStorage<Self>;
}