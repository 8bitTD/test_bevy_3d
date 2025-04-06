use bevy::{
    prelude::*, 
    window::*,
};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
mod define;
mod fps;
mod state;

fn main() {
    //let mut rp = RapierDebugRenderPlugin::default();
    //rp.enabled = define::debug::RAPIERDEBUGRENDERPLUGINENABLED;
    let px = get_position_x();
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window {
                    title: define::common::TOOLNAME.into(),
                    position: WindowPosition::new(IVec2::new(px, 0)),
                    resolution: (1500.0, 900.0).into(),
                    //present_mode: PresentMode::AutoNoVsync, 
                    //present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                exit_condition: bevy::window::ExitCondition::OnAllClosed,
                close_when_requested: true,
                ..default()
            },
        ).set(bevy::log::LogPlugin{
            level: bevy::log::Level::WARN,
            ..default()
        }).set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(bevy_egui::EguiPlugin)
        //.add_plugins(WorldInspectorPlugin::new())       
        .add_plugins(state::StatePlugin)
        .add_plugins(fps::FPSPlugin::new(1))
        .run();
}

#[cfg(target_arch = "wasm32")]
pub fn get_position_x() -> i32{
    0
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_position_x() -> i32{
    -1700
}
