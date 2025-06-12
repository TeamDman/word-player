use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::bevy_egui::EguiGlobalSettings;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use clap::Parser;
use exit_on_esc::ExitOnEscPlugin;
use setup_windows::SetupWindowsPlugin;
use world_inspector_plugin::YMBWorldInspectorPlugin;

mod dimmer_plugin;
mod exit_on_esc;
mod prompt_draw_plugin;
mod region_selection_overlay_plugin;
mod region_selection_plugin;
mod region_selection_return_plugin;
mod setup_windows;
mod cursor_position_plugin;
mod world_cursor_text_plugin;
mod world_inspector_plugin;

#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show debug output
    #[arg(long)]
    debug: bool,
    /// Optional prompt text to display
    #[arg(long)]
    prompt: Option<String>,
    /// Enable world inspector window
    #[arg(long)]
    inspector: bool,
}

fn main() {
    let args = Args::parse();
    let mut app = App::new();
    app.insert_resource(args)
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(EmbeddedAssetPlugin {
            mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(
                Window {
                    decorations: false,
                    transparent: true,
                    ..default()
                }
            ),
            ..default()
        }))
        .add_plugins(SetupWindowsPlugin)
        .add_plugins(ExitOnEscPlugin)
        .add_plugins(region_selection_plugin::RegionSelectionPlugin)
        .add_plugins(region_selection_return_plugin::RegionSelectionReturnPlugin)
        .add_plugins(region_selection_overlay_plugin::RegionSelectionOverlayPlugin)
        .add_plugins(prompt_draw_plugin::PromptDrawPlugin)
        .add_plugins(dimmer_plugin::DimmerPlugin)
        .add_plugins(cursor_position_plugin::CursorPositionPlugin)
        .add_plugins(world_cursor_text_plugin::WorldCursorTextPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(Startup, |_config: ResMut<EguiGlobalSettings>| {
            // config.enable_absorb_bevy_input_system = true
        });
    if app.world().resource::<Args>().inspector {
        app.add_plugins(YMBWorldInspectorPlugin);
    }
    app.run();
}
