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
    /*
    pub stage_count: usize,
    pub game_state: GameState,
    pub game_state_timer: f32,
    pub rope_distance: f32,
    pub is_reset_game: bool,
    pub text_stage_alpha: f32,
    pub is_clear: bool,
    pub timer: f32,
    pub grab_count: usize,
    pub is_ending_end: bool,
    pub is_tutorial_skip_button_hover: bool,
    pub is_tutorial_reset_button_hover: bool,
    pub tutorial_grab_blink_timer: f32,
    pub tutorial_mouse_move_timer: f32,
    pub continues: usize,
    */
}
impl Default for MyApp{
    fn default() -> MyApp{
        MyApp { 
            /*
            stage_count: debug::STARTSTAGE,
            game_state: GameState::In,
            game_state_timer: 0.0,
            rope_distance: value::DEFAULTROPEDISTANCE, 
            is_reset_game: false,
            text_stage_alpha: value::DEFAULTTEXTSTAGEALPHA,
            is_clear: debug::ISCLEAR,
            timer: 0.0,
            grab_count: 0,
            is_ending_end: false,
            is_tutorial_skip_button_hover: false,
            is_tutorial_reset_button_hover: false,
            tutorial_grab_blink_timer: 0.0,
            tutorial_mouse_move_timer: 0.0,
            continues: 0,
            */
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
                game::update_camera,
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
