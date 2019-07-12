// amethyst modules
use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Clone, new)]
struct UiJumpingComp {
    group:      String,     // jumping group
    order:      usize,      // jumping order, 
    interval:   f32,        // jumping interval
    height:     f32,        // jumping height
}

// make it component
impl Component for UiJumpingComp {
    type Storage = DenseVecStorage<Self>;
}