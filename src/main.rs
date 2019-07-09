//================
// Import modules
//================

// external macros
#[macro_use]
extern crate log;

// general modules
use std::time::Duration;

// amethyst modules
use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::prelude::{ReadExpect, Resources, SystemData},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        pass::DrawFlat2DDesc,
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{format::Format, image},
        },
        types::DefaultBackend,
        GraphCreator, RenderingSystem, SpriteSheet
    },
    ui::{DrawUiDesc, UiBundle},
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
    assets::Processor,
    LogLevelFilter
};

//======================
// Import local modules
//======================
mod states;
mod systems;
mod components;
use crate::states::LoadingState;
use crate::states::{CustomStateEvent, CustomStateEventReader};

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
    let app_root = application_root_dir()?;

    // construct the resources_dir by using app_root + "resources"
    let resources_dir = app_root.join("resources");

    // construct the display_config_path by using resources_dir + "display_confg.ron"
    let display_config_path = resources_dir.join("display_config.ron");

    // constrcut the controller configuration path
    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            app_root.join("resources/input_controller.ron")
        } else {
            app_root.join("resources/input.ron")
        }
    };

    // create a default game data with
    // with bundle "windowBundle" which constructed from display_config_path
    //
    // with bundle "TransformBundle" which handles tracking entity positions
    // with thread local "RenderingSystem" with the created default RenderingGraph
    //             The renderer must be executed on the same thread consecutively,
    //             so we initialize it as thread_local which will always execute on the main thread.
    let game_data = GameDataBuilder::default()
        // The WindowBundle provides all the scaffolding for opening a window
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        // Input bundle to hanlde input, wkth the key binding configuratioin
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        // UI bundle to handle UI
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        // Add user defined systems
        .with(systems::UiFlashingSystem, "text_flashing_system", &[])
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
            RenderingGraph::default(),
        ));

    // create an Application "game"
    // with resource directory "resources_dir",
    // instance of the loading state "LoadingState",
    // and the "game_data" just created
    // note that use this type of definition is because we are using custom state event.
    // for default event types the follow way should be used
    //      let mut game = Application::new(assets_dir, state_name, game_data)?;
    let mut game: CoreApplication<GameData, CustomStateEvent, CustomStateEventReader> =
        CoreApplication::build(resources_dir, LoadingState::default())?
            .with_frame_limit(
                FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
                60,
            )
            .build(game_data)?;

    // run the game,  this will start the game loop
    game.run();

    // when the game exit, return OK with ().
    Ok(())
}

//========================
// Define Rendering Graph
//========================
//
// use #[derive(Default)] on a data structure,
// the compiler will automatically create a default function for you
// that fills each field with its default value. (if the type has Default trait implemented)
// The default boolean value is false, the default integral value is 0.
//
#[derive(Default)]
struct RenderingGraph {
    dimensions: Option<ScreenDimensions>, // windows dimensions for tracking
    surface_format: Option<Format>,       // cached surface format
    dirty: bool,                          // default set to false
}

//=================================
// Implement Graphic Creator Trait
//=================================
impl GraphCreator<DefaultBackend> for RenderingGraph {

    //----------------------------
    // Implement rebuild function
    //----------------------------
    //
    // This trait method reports to the renderer if the graph must be rebuilt, usually because
    // the window has been resized. This implementation checks the screen size and returns true
    // if it has changed.
    // (Boilerplate code? might be simiplied in later version)
    //
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.

        // try to get id="ScreenDimensions" resource from the resouces container
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        // import Deref Module
        use std::ops::Deref;
        // check the dimention changed or not by comparing them (ScreenDimensions implemented PartialEq)
        // left:  use as_ref() to retrieve ScreenDimensions inside the option "self.dimensions"
        // right: use as_ref() to retrieve Fetch<ScreenDimensions> inside the option "new_dimensions"
        //        then use map to take it and deref it to ScreenDimensions (Fetch implemented Deref)
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            // if the dimensions are changed
            // set the dirty to true, so that if there is no more change after this frame,
            // next frame will guaranteed to be rebuilt
            self.dirty = true;
            // store the new dimensions
            self.dimensions = new_dimensions.map(|d| d.clone());
            // return false to skip the rebuild for this frame
            return false;
        }

        // If demension is not changing any more rebuild the graph
        self.dirty
    }

    //----------------------------
    // Implement builder function
    //----------------------------
    //
    // This is the core of a RenderGraph, which is building the actual graph with subpasses and target
    // images.
    //
    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        // import rendy stuff
        use amethyst::renderer::rendy::{
            graph::present::PresentNode,
            hal::command::{ClearDepthStencil, ClearValue},
        };

        // Grpah is being rebuilt, so reset the dirty back to false
        self.dirty = false;
        // Retrieve a reference to the target window, which is created by the WindowBundle
        // <ReadExpect<'_, Window>> is a panic version of Read
        // fetch method will fetch the resource with the type "Window" or panics if it doesn't exist.
        // so once completed, window should be Read<Window> type (??)
        let window = <ReadExpect<'_, Window>>::fetch(res);
        // use the window recource to create a Rendy surface
        let surface = factory.create_surface(&window);
        // cache surface format to speed things up
        // the get_or_insert_with method will only do once if it is successful.
        // since the device is not changed, this suface value is cached here.
        let surface_format = *self
            .surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));
        // get a reference to the dimensions
        let dimensions = self.dimensions.as_ref().unwrap();
        // construct a Kind type varaible "window_kind"
        let window_kind =
            image::Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        // get a graphBuilder instance
        let mut graph_builder = GraphBuilder::new();
        // create a 2D image coverred the entire frame with a certain color
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.34, 0.36, 0.52, 1.0].into())),
        );

        // create a "depth stencil" image
        // sort of a default one (?)
        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        // Create our first `Subpass`, which contains the DrawFlat2D and DrawUi render groups.
        // We pass the subpass builder a description of our groups for construction
        let pass = graph_builder.add_node(
            // creating the render pass using SubpassBuilder
            // with_group 1:       create a simple 2d pass
            // with_group 2:       create a UI pass
            // with color:         use the 2d color image just created
            // with depth_stencil: use the default depth stencil just created
            // into_pass() will convert the subpass to a pass
            SubpassBuilder::new()
                .with_group(DrawFlat2DDesc::new().builder()) // Draws sprites
                .with_group(DrawUiDesc::new().builder())     // Draws UI components
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        // Finally, add the pass to the graph.
        // The PresentNode takes its input and applies it to the surface.
        // use a "unused" variable _present to store the value.
        let _present = graph_builder
            .add_node(PresentNode::builder(factory, surface, color).with_dependency(pass));

        // return the graph_builder just contructed
        graph_builder
    }
}
