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
    assets::Processor,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        types::DefaultBackend,
        RenderingSystem, SpriteSheet,
    },
    ui::UiBundle,
    utils::application_root_dir,
    window::WindowBundle,
    LogLevelFilter,
};

//======================
// Import local modules
//======================
mod components;
mod states;
mod systems;
mod render_graph;
mod resources;
mod mx_utils;
use crate::render_graph::RenderGraph;
use crate::states::loading_state::LoadingState;
use crate::systems::ps_ui_bundle::PsUiBundle;

//===============
// main function
//===============
fn main() -> amethyst::Result<()> {
    // start the logger with less vulkan related junks and
    // please use amethyst:start_logger(Default::default()) instead if vulkan is shut up in the future
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

    // display_config_path = resources_dir + "display_confg.ron"
    let display_config_path = resources_dir.clone() + "/display_config.ron";

    // input configuration path
    let key_bindings_path = resources_dir.clone() + "/input.ron";


    // The global game data. Here we register all systems and bundles that will run for every game state.
    // The game states will define additional dispatchers for state specific systems.
    // Note that the dispatchers will run in sequence,
    // so this setup sacrifices performance for modularity (for now).
    let game_data = GameDataBuilder::default()
        // Input bundle to hanlde input, with the key binding configuratioin
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        // The WindowBundle provides all the scaffolding for opening a window
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        // UI bundle to handle UI
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        // Add user defined UI systems
        .with_bundle(PsUiBundle)?
        // Sprite sheet processor have to be loaded when DrawFlat2DDesc pass is used,
        // or the program will panic "Tried to fetch a resource, but the resource does not exist."
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ));

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