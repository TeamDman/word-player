use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiGlobalSettings, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::Parser;
use exit_on_esc::ExitOnEscPlugin;
use setup_windows::SetupWindowsPlugin;
use world_inspector_plugin::YMBWorldInspectorPlugin;

mod exit_on_esc;
mod prompt_draw_plugin;
mod region_selection_plugin;
mod setup_windows;
mod dimmer_plugin;
mod world_inspector_plugin;

/// User Region Identification and Naming Exosystem
#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show debug output
    #[arg(long)]
    debug: bool,
    /// Optional prompt text to display
    #[arg(long)]
    prompt: Option<String>,
}

fn main() {
    let args = Args::parse();
    App::new()
        .insert_resource(args)
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None, // We'll create windows for all monitors
            ..default()
        }))
        .add_plugins(SetupWindowsPlugin)
        .add_plugins(ExitOnEscPlugin)
        .add_plugins(region_selection_plugin::RegionSelectionPlugin)
        .add_plugins(prompt_draw_plugin::PromptDrawPlugin)
        .add_plugins(dimmer_plugin::DimmerPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(Startup, |mut config: ResMut<EguiGlobalSettings>| {
            // config.enable_absorb_bevy_input_system = true
        })
        .add_plugins(YMBWorldInspectorPlugin)
        .run();
}
