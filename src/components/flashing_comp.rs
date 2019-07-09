//================
// Import modules
//================

// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

//================
// Flashing Style
//================
#[derive(Clone)]
#[allow(dead_code)]
pub enum FlashingStyle {
    TwoWays,        // color get lightened and darkened
    Lightening,     // color only get lightened
    Darkening,      // color only get darkened
}

//================
// Flashing Style
//================
#[derive(Clone)]
pub struct FlashingComp {
    pub orginal_color:  [f32; 4],       // Saved original color of the text
    pub is_flashing:    bool,           // control if the text is flashing or not
    pub rate:           f32,            // flashing rate (default is 1.0)
    pub intensity:      f32,            // flashing intensity (from 0.0 to 2.0, default is 0.4)
    pub style:          FlashingStyle,  // flashing style
    pub rgba_factors:   [f32; 4],       // rgba channel factors
}

impl FlashingComp {
    pub fn new(
        orginal_color:  [f32; 4], 
        is_flashing:    bool, 
        rate:           f32, 
        intensity:      f32, 
        style:          FlashingStyle, 
        rgba_factors:   [f32; 4],
    ) -> Self {
        FlashingComp {
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
impl Default for FlashingComp {
    fn default() -> Self {
        FlashingComp {
            orginal_color:  [0.6, 0.6, 0.6, 0.6,],
            is_flashing:    true,
            rate:           1.,
            intensity:      0.4,
            style:          FlashingStyle::TwoWays,
            rgba_factors:   [1., 1., 1., 0.,],
        }
    }
}

// make it component
impl Component for FlashingComp {
    type Storage = DenseVecStorage<Self>;
}