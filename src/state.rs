use bevy::prelude::*;
//use serde::{Serialize, Deserialize};
//use std::io::Write;

pub mod game;

use super::define::*;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState{
    #[default]
    Game,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState{
    #[default]
    Game,
}


#[derive(Resource)] 
pub struct MyApp{
    pub is_reset_game: bool,
}
impl Default for MyApp{
    fn default() -> MyApp{
        MyApp { 
            is_reset_game: false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Resource)]
pub enum GameState{
    #[default]
    In,
    //Play,
    //Out,
}

#[derive(Component)]
pub struct ReleaseResource;//リソース開放用
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build (&self, app: &mut App){
        app
        .init_state::<AppState>()
        .insert_resource(MyApp::default())
        .add_systems(OnEnter(AppState::Game), (
            game::setup_asset,
        ))
        .add_systems(Update, 
            (
                game::update_player,
                game::update_camera.after(game::update_player),
                game::check_reset_game,
                game::gizmo,
            ).chain().run_if(in_state(AppState::Game)),
        )
        .add_systems(OnExit(AppState::Game), despawn);
        
    }
}

pub fn despawn(
    mut commands: Commands, 
    query: Query<Entity, With<ReleaseResource>>,
){
    for entity in &mut query.iter() {
        commands.entity(entity).try_despawn_recursive();
    }
}
