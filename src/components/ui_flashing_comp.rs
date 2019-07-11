// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

//================
// Flashing Style
//================
#[derive(Clone)]
#[allow(dead_code)]
pub enum UiFlashingStyle {
    TwoWays,        // color get lightened and darkened
    Lightening,     // color only get lightened
    Darkening,      // color only get darkened
}

//====================
// Flashing Component
//====================
#[derive(Clone)]
pub struct UiFlashingComp {
    pub orginal_color:  [f32; 4],       // Saved original color
    pub is_flashing:    bool,           // control if the ui element is flashing or not
    pub rate:           f32,            // flashing rate (default is 1.0)
    pub intensity:      f32,            // flashing intensity (from 0.0 to 2.0, default is 0.4)
    pub style:          UiFlashingStyle,// flashing style
    pub rgba_factors:   [f32; 4],       // rgba channel factors
}

impl UiFlashingComp {
    pub fn new(
        orginal_color:  [f32; 4], 
        is_flashing:    bool, 
        rate:           f32, 
        intensity:      f32, 
        style:          UiFlashingStyle, 
        rgba_factors:   [f32; 4],
    ) -> Self {
        UiFlashingComp {
            orginal_color,
            is_flashing,
            rate,
            intensity,
            style,
            rgba_factors,   
        }
    }
}

// default values
impl Default for UiFlashingComp {
    fn default() -> Self {
        UiFlashingComp {
            orginal_color:  [0.6, 0.6, 0.6, 0.6,],
            is_flashing:    true,
            rate:           1.,
            intensity:      0.4,
            style:          UiFlashingStyle::TwoWays,
            rgba_factors:   [1., 1., 1., 0.,],
        }
    }
}

// make it component
impl Component for UiFlashingComp {
    type Storage = DenseVecStorage<Self>;
}