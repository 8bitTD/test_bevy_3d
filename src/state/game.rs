use bevy::{
    prelude::*,
    input::{mouse::MouseMotion, InputSystem},
    utils::Duration,
};
use bevy_rapier3d::prelude::*;
use rand::distributions::{Distribution, Uniform};
//use bevy_rapier2d::prelude::*;
//use rapier2d::prelude::RigidBodyChanges;
//use rapier2d::prelude::RigidBodyType;
//use rand::distributions::{Distribution, Uniform};


//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use super::super::define::*;
use super::super::state::*;

const MOUSE_SENSITIVITY: f32 = 0.3;
const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 20.0;
const GRAVITY: f32 = -9.81;

#[derive(Component)]
pub struct PlayerRoot;
#[derive(Component)]
pub struct PlayerTurn;
#[derive(Component)]
pub struct PlayerInfo;
#[derive(Component)]
pub struct PlayerCameraPos;

#[derive(Component)]
pub struct MissileInfo;
/*
/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct MovementInput(Vec3);
/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
*/
pub struct LookInput(Vec2);

pub fn gizmo(
    mut gizmos: Gizmos,
) {
    gizmos.grid(
        Quat::from_rotation_x(90.0_f32.to_radians()),
        UVec2::splat(110),
        Vec2::new(10.0, 10.0),
        Color::srgba_u8(0, 0, 0, 64)
    );

    //gizmos.grid_3d(
    //    Quat::from_rotation_x(90.0_f32.to_radians()),
    //    UVec3::splat(10),
    //    Vec3::new(100.0,100.0, 100.0),
    //    Color::srgba_u8(0, 0, 64, 64)
    //);
}



pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_root: Single<&mut Transform, With<PlayerRoot>>,
    mut player_turn: Single<&mut Transform, (With<PlayerTurn>, Without<PlayerRoot>)>,
    mut app: ResMut<MyApp>,
    time: Res<Time>,
) {
    let ds = time.delta_secs();

    let left = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let down = keyboard.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let up = keyboard.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    if left{
        player_turn.rotation.z = 20.0_f32.to_radians();
        player_root.rotate_local_y(100.0_f32.to_radians() * ds);
    }
    if right{
        player_turn.rotation.z = -20.0_f32.to_radians();
        player_root.rotate_local_y(-100.0_f32.to_radians() * ds);
    }
    if up {
        player_turn.rotation.x = 5.0_f32.to_radians();
        player_root.translation.y += 20.0 * ds;
    }
    if down && player_root.translation.y > 2.0{
        player_turn.rotation.x = -5.0_f32.to_radians();
        player_root.translation.y -= 20.0 * ds;
    }
    if !left && !right && !up && !down{
        player_turn.rotation = Quat::from_rotation_x(0.0_f32.to_radians());
        player_turn.rotation = Quat::from_rotation_y(0.0_f32.to_radians());
        player_turn.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
    }
    let dir = player_root.forward() * ds * system::FPS;
    player_root.translation += dir;
}

pub fn player_movement(
    time: Res<Time>,
    mut app: Res<MyApp>,
    mut player: Single<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
) {
    let delta_time = time.delta_secs();
    let tmp = player.0.rotation * (app.movement * delta_time);
    player.1.translation = Some(tmp);
}



pub fn setup_asset(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    for x in -2..3{
        for y in 1..3{
            for z in -2..3{
                commands.spawn((
                    Transform::from_xyz( x as f32 * 40.0, y as f32 * 40.0, z as f32 * 40.0 - 300.0),
                    Mesh3d(meshes.add(Cuboid::new(10.0, 10.0, 10.0))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
                    MissileInfo,
                ));
            }
        }
    }

    commands.spawn((
        Transform::from_xyz(0.0, 5.0, 0.0),
        //Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        //MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
        Visibility::default(),
        Collider::round_cylinder(0.3, 0.3, 0.3),
        KinematicCharacterController {
            custom_mass: Some(5.0),
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            slide: true,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.3),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: false,
            }),
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0_f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: None,
            ..default()
        },
        PlayerRoot,
    )).with_children(|b| {
        b.spawn((
            Transform::from_xyz(0.0, -0.5, 0.0),
            PlayerTurn,
        )).with_children(|c|{
            c.spawn((//本体
                Mesh3d(meshes.add(Tetrahedron::new(
                    Vec3::new(-1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0), 
                    Vec3::new(0.0,0.0, -6.0), 
                    Vec3::new(0.0, 1.0, 0.0)
                ))),
                MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
                Transform::from_xyz(0.0, 0.0, 0.0),
                PlayerInfo,
            ));
            c.spawn((//左翼
                Mesh3d(meshes.add(Tetrahedron::new(
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, -2.0), 
                    Vec3::new(4.0,0.0, 1.0), 
                    Vec3::new(1.5, 0.25, 0.0)
                ))),
                MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            c.spawn((//右翼
                Mesh3d(meshes.add(Tetrahedron::new(
                    Vec3::new(-1.0, 0.0, 0.0),
                    Vec3::new(-1.0, 0.0, -2.0), 
                    Vec3::new(-4.0,0.0, 1.0), 
                    Vec3::new(-1.5, 0.25, 0.0)
                ))),
                MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        });
        // FPS Camera
        b.spawn((
            Camera3d::default(), 
            Projection::from(PerspectiveProjection {
                //fov: 0.78,
                fov: 1.00,
                ..default()
            }),
            Transform::from_xyz(0.0, 1.5, 15.0),
        ));
    });



    commands.spawn((//バージョン表記
        Text::new(env!("CARGO_PKG_VERSION")),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 10.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::End,
            top: Val::Px(0.0),
            ..default()
        },
        ReleaseResource,
    ));
}
