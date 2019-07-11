// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

//==================
// Cursor Positioin
//==================
#[derive(Clone)]
#[allow(dead_code)]
pub enum UiMainMenuCursorPos {
    Game1Player,
    Game2Player2,
    GameCpuVCpu,
    GameExit,
}

impl UiMainMenuCursorPos {
    pub fn next_pos(&self, ascending:bool) -> UiMainMenuCursorPos {
        if ascending {
            match self {
                UiMainMenuCursorPos::Game1Player  => UiMainMenuCursorPos::Game2Player2,
                UiMainMenuCursorPos::Game2Player2 => UiMainMenuCursorPos::GameCpuVCpu,
                UiMainMenuCursorPos::GameCpuVCpu  => UiMainMenuCursorPos::GameExit,
                UiMainMenuCursorPos::GameExit     => UiMainMenuCursorPos::Game1Player,
            }
        } else {
            match self {
                UiMainMenuCursorPos::Game1Player  => UiMainMenuCursorPos::GameExit,
                UiMainMenuCursorPos::Game2Player2 => UiMainMenuCursorPos::Game1Player,
                UiMainMenuCursorPos::GameCpuVCpu  => UiMainMenuCursorPos::Game2Player2,
                UiMainMenuCursorPos::GameExit     => UiMainMenuCursorPos::GameCpuVCpu,
            }
        }
    }

    pub fn get_local_y(&self) -> f32 {
        match self {
            UiMainMenuCursorPos::Game1Player  =>  0.,
            UiMainMenuCursorPos::Game2Player2 => -100.,
            UiMainMenuCursorPos::GameCpuVCpu  => -200.,
            UiMainMenuCursorPos::GameExit     => -350.,
        }       
    }
}

//=================
// Cursor Movement
//=================
#[derive(Clone)]
pub struct UiCursorMovementComp {
    pub current_pos:    UiMainMenuCursorPos,
}

#[allow(dead_code)]
impl UiCursorMovementComp {
    pub fn set_pos(&mut self, new_pos:UiMainMenuCursorPos) {
        self.current_pos = new_pos;
    }

    pub fn advance_pos(&mut self, direction:bool) {
        self.current_pos = self.current_pos.next_pos(direction);
    }
}

// default values
impl Default for UiCursorMovementComp {
    fn default() -> Self {
        UiCursorMovementComp {
            current_pos:  UiMainMenuCursorPos::Game1Player,
        }
    }
}

// make it component
impl Component for UiCursorMovementComp {
    type Storage = DenseVecStorage<Self>;
}