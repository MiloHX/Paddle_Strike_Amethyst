//=========================
// Import amethyst modules
//=========================
use amethyst::{
    ecs::prelude::{
        System,
        Join,
        ReadStorage,
        WriteStorage
    },
    ui::{
        UiText,
        UiTransform
    }
};

//========================
// Declare loading system
//========================
pub struct LoadingSystem;

//========================
// Implement System trait
//========================
impl<'s> System<'s> for LoadingSystem {
    type SystemData = (
        ReadStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>
    );

    fn run(&mut self, (texts, mut trans): Self::SystemData) {
        for (_, mut tran) in (&texts, &mut trans).join() {
            if tran.local_x > -200. {
                tran.local_x -= 0.5;
            } else {
                tran.local_x = 0.;
            }
        }
    }
}