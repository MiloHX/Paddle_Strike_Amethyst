use std::iter::Cycle;
use std::vec::IntoIter;
use std::ops::Deref;

use amethyst::{
    assets::{
        AssetStorage,
        Loader,
    },
    audio::{
        output::Output,
        AudioSink,
        WavFormat,
        Source,
        SourceHandle,
    },
    ecs::prelude::World,
};

const MENU_MUSIC:       &'static [&'static str] = &["assets/sounds/theme.wav"];
const SFX_CURSOR_TICK:  &str = "assets/sounds/hover.wav";
const SFX_BUTTON_PUSH:  &str = "assets/sounds/push.wav";

pub enum SoundType {
    CursorTick,
    ButtonPush,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

pub struct Sounds {
    pub cursor_tick: SourceHandle,
    pub button_push: SourceHandle,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, WavFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.5);
        sink.pause();
        


        let music = MENU_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, &file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        let music = Music { music };

        let sound = Sounds {
            cursor_tick: load_audio_track(&loader, &world, SFX_CURSOR_TICK),
            button_push: load_audio_track(&loader, &world, SFX_BUTTON_PUSH),
        }; 

        (sound, music)
    };

    world.add_resource(sound_effects);
    world.add_resource(music);
}

pub fn play_sound(sound_type: SoundType, sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        match sound_type {
            SoundType::CursorTick => {
                if let Some(sound) = storage.get(&sounds.cursor_tick) {
                    output.play_once(sound, 0.5);
                }
            }
            SoundType::ButtonPush => {
                if let Some(sound) = storage.get(&sounds.button_push) {
                    output.play_once(sound, 0.5);
                }               
            }
        }
    }
}

pub fn play_sfx(sound_type: SoundType, world: &mut World) {
    let sounds       = world.read_resource::<Sounds>();
    let storage      = world.read_resource::<AssetStorage<Source>>();
    let audio_output = Some(world.read_resource::<Output>());
    play_sound(
        sound_type, 
        &*sounds,
        &storage,
        audio_output.as_ref().map(|o| o.deref()),
    );
}

pub fn pause_music(world: &mut World) {
    let sink = world.write_resource::<AudioSink>();
    sink.pause();
}

pub fn resume_music(world: &mut World) {
    let sink = world.write_resource::<AudioSink>();
    sink.play();
}

