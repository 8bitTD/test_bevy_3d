use bevy::{
    prelude::*,
    utils::Duration,
};
use bevy_rapier3d::prelude::*;
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
    pub missile_timer: Timer,
    pub movement: Vec3,
}
impl Default for MyApp{
    fn default() -> MyApp{
        MyApp { 
            is_reset_game: false,
            missile_timer: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
            movement: Vec3::new(0.0, 0.0, 0.0),
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
                game::handle_input,
                game::gizmo,
            )
        )
        .add_systems(FixedUpdate, game::player_movement)
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
