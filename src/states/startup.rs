//=========================
// Import amethyst modules
//=========================
use amethyst::prelude::*;

//=======================
// Declare startup state
//=======================
//
// Note that if it is not a unit struct (with no fields)
// you cannot directly use it as the parameter of the Application::new() function
// a seperate method to return an instance (Self) need to be used
pub struct StartUpState {
    startup_time: f32
}

//=========================
// Implement startup state
//=========================
impl StartUpState {
    // return and instance of itself with default values
    pub fn new() -> Self {
        StartUpState {
            startup_time: 3.0
        }
    }
}

//=============================
// Implement SimpleState trait
//=============================
impl SimpleState for StartUpState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        // To be replaced by meaning logic
        println!("Startup time: {}", self.startup_time);
    }
}