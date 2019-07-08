//====================================
// Modules declaration for components
//====================================
//
// An Entity represents a single object in your world. 
// Component represents one aspect of an object. 
// For example, a bottle of water has a shape, a volume, a color and is made of a material (usually plastic). 
// In this example, the bottle is the entity, and the properties are components.
//

// declare modules
mod flashing_comp;

// move the struts / enums inside the modules into this scope ("components")
pub use self::flashing_comp::FlashingComp;
pub use self::flashing_comp::FlashingStyle;