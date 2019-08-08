//================
// Import modules
//================

// external macros
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_new;

// amethyst modules
use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        types::DefaultBackend,
        RenderingBundle,
        plugins::{
            RenderFlat2D, 
            RenderToWindow,
        },
    },
    ui::{
        UiBundle, 
        RenderUi,
    },
    audio::{
        AudioBundle,
        DjSystemDesc,
    },
    utils::application_root_dir,
    LogLevelFilter,
};

//======================
// Import local modules
//======================
mod components;
mod states;
mod systems;
mod resources;
mod mx_utils;
use crate::states::loading_state::LoadingState;
use crate::systems::ps_ui_bundle::PsUiBundle;
use crate::resources::audio::Music;

//===============
// main function
//===============
fn main() -> amethyst::Result<()> {
    // start the logger with less vulkan related junks
    // use amethyst:start_logger(Default::default()) instead if vulkan is shut up in the future
    amethyst::Logger::from_config(amethyst::LoggerConfig {
        level_filter: LogLevelFilter::Info,
        ..Default::default()
    })
    .level_for("gfx_backend_vulkan", LogLevelFilter::Warn)
    .start();

    // save the application root to app_root
    let resources_dir = application_root_dir()?
        .into_os_string()
        .into_string()
        .unwrap()
        + "/resources";

    // display configuration path
    let display_config_path = resources_dir.clone() + "/display_config.ron";

    // input configuration path
    let key_bindings_path = resources_dir.clone() + "/input.ron";

    // The global game data. Here we register all systems and bundles that will run for every game state.
    let game_data = GameDataBuilder::default()
        // input bundle to hanlde input, with the key binding configuratioin
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        // transform bundle handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        // audio bundle handle audio
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        // UI bundle handles UI
        .with_bundle(UiBundle::<StringBindings>::new())?
        // Add user defined UI systems
        .with_bundle(PsUiBundle)?
        // Add rendering graph
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                // render 2D
                .with_plugin(RenderFlat2D::default())
                // render ui
                .with_plugin(RenderUi::default()),
        )?;

    // create an Application "game"
    // with resource directory "resources_dir",
    // instance of the loading state "LoadingState",
    // and the "game_data" just created
    let mut game: Application<GameData> =
        Application::build(resources_dir, LoadingState::default())?
            .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
            .build(game_data)?;

    // run the game,  this will start the game loop
    game.run();

    // when the game exit, return OK with ().
    Ok(())
}