//=================================
// Modules declaration for systems
//=================================
//
// A system is where the logic of the game is executed. In practice, 
// it consists of a struct implementing a function executed on every iteration of the game loop, 
// and taking as an argument data about the game.
// Systems can be seen as a small unit of logic. All systems are run by the engine together
// (even in parallel when possible), and do a specialized operation on one or a group of entities.
//

// declare modules
mod loading_system;

// move the struts / enums inside the modules into this scope ("systems")
pub use self::loading_system::LoadingSystem;