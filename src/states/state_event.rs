//=========================
// Import amethyst modules
//=========================
// import macros
use amethyst::derive::EventReader;
// macro dependancies
use amethyst::core::{
    ecs  ::{Read, SystemData, Resources},
    shrev::{ReaderId, EventChannel},
    EventReader
};
// events
use amethyst::{
    winit::Event,
    input::InputEvent,
    ui   ::UiEvent
};
//====================================
// Define Custom Event for the states
//====================================
//
// #[derive(Debug)]
// asks the compiler to auto-generate a suitable implementation of the Debug trait, 
// which provides the result of {:?} in something like 
// format!("Would the real {:?} please stand up!", Person { name: "John", age: 8 });
//
// #[derive(Clone)]
// A common trait for the ability to explicitly duplicate an object.
// Differs from Copy in that Copy is implicit and extremely inexpensive, 
// while Clone is always explicit and may or may not be expensive. 
// In order to enforce these characteristics, Rust does not allow you to reimplement Copy, 
// but you may reimplement Clone and run arbitrary code.
// Since Clone is more general than Copy, you can automatically make anything Copy be Clone as well.
//
// #[derive(EventReader)]
// The default StateEvent regroups multiple types of events that are emitted throughout the engine by default.
// To change the set of events that the state receives, you create a new event enum 
// and derive EventReader for that type.
//
// #[reader(xxxEventReader)]
// To make Application aware of the change to which events to send to the state, 
// you also need to supply both the event type, and the EventReader type 
// (the name you give in the #[reader(SomeReader)] derive attribute) 
// when the Application is created. This is done by replacing 
//      Application::build (or Application::new) 
//      with 
//      CoreApplication::<_, MyEvent, MyEventReader>::build() 
//  (or CoreApplication::<_, MyEvent, MyEventReader>::new()   ).
// 

#[derive(Debug, EventReader, Clone)]
#[reader(CustomStateEventReader)]
pub enum CustomStateEvent {
    Window(Event),
    Ui(UiEvent),
    Input(InputEvent<String>)
}