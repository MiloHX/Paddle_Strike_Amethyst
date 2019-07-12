// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

//========
// Cursor 
//========
#[derive(Clone, new, Default)]
pub struct UiCursorComp {
    pub group:          String,
    pub current_pos:    usize,
    pub pos_list:       Vec<(f32, f32)>,
    pub pos_id_list:    Vec<String>,
}

#[allow(dead_code)]
impl UiCursorComp {
    pub fn set_pos(&mut self, new_pos:usize) {
        self.current_pos = new_pos;
    }

    pub fn advance_pos(&mut self, direction:bool) {
        if direction {
            if self.current_pos < self.pos_list.len() - 1 {
                self.current_pos += 1;
            } else {
                self.current_pos = 0;
            }
        } else {
            if self.current_pos > 0 {
                self.current_pos -= 1;
            } else {
                self.current_pos = self.pos_list.len() - 1;
            }
        }
    }
}

// make it component
impl Component for UiCursorComp {
    type Storage = DenseVecStorage<Self>;
}