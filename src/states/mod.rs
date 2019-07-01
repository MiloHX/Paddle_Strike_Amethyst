//================================
// Modules declaration for states
//================================
//
// States are only valid for a certain period of time, during which a lot of things can occur.
// A State contains methods that reflect the most common of those events:
// 
//      on_start:           When a State is added to the stack, this method is called on it.
// 
//      on_stop:            When a State is removed from the stack, this method is called on it.
// 
//      on_pause:           When a State is pushed over the current one, the current one is paused, 
//                          and this method is called on it.
// 
//      on_resume:          When the State that was pushed over the current State is popped, 
//                          the current one resumes, and this method is called on the now-current State.
// 
//      handle_event:       Allows easily handling events, like the window closing or a key being pressed.
// 
//      fixed_update:       This method is called on the active State at a fixed time interval 
//                          (1/60th second by default).
// 
//      update:             This method is called on the active State as often as possible by the engine.
// 
//      shadow_update:      This method is called as often as possible by the engine on all States 
//                          which are on the StateMachines stack, including the active State. 
//                          Unlike update, this does not return a Trans.
// 
//      shadow_fixed_update:This method is called at a fixed time interval (1/60th second by default) 
//                          on all States which are on the StateMachines stack, 
//                          including the active State. Unlike fixed_update, this does not return a Trans.
// 
// If you aren't using SimpleState or EmptyState, you must implement the update method 
// to call data.data.update(&mut data.world).

// declare modules
mod loading_state;
mod state_event;    

// move the struts / enums inside the modules into this scope ("states")
pub use self::loading_state::LoadingState;
pub use self::state_event::CustomStateEvent;
pub use self::state_event::CustomStateEventReader;
