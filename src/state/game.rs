use bevy::{
    prelude::*, 
    //color::palettes::basic,
    //sprite::*, 
    //audio,
};
//use bevy_rapier2d::prelude::*;
//use rapier2d::prelude::RigidBodyChanges;
//use rapier2d::prelude::RigidBodyType;
//use rand::distributions::{Distribution, Uniform};


//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use super::super::define::*;
use super::super::state::*;

#[derive(Component)]
pub struct PlayerRoot;
#[derive(Component)]
pub struct PlayerTurn;
#[derive(Component)]
pub struct PlayerInfo;
#[derive(Component)]
pub struct PlayerCameraPos;

pub fn draw_example_collection(
    mut gizmos: Gizmos,
) {
    gizmos.grid(
        Quat::from_rotation_x(90.0_f32.to_radians()),
        UVec2::splat(105),
        Vec2::new(10.0, 10.0),
        Color::srgba_u8(0, 0, 0, 64)
    );
}

pub fn check_reset_game(
    mut player_root: Single<&mut Transform, With<PlayerRoot>>,
    mut app: ResMut<MyApp>,
){
    if !app.is_reset_game{return;}
    player_root.translation.x = 0.0;
    player_root.translation.y = 2.0;
    player_root.translation.z = 0.0;
    player_root.rotation = Quat::from_rotation_y(0.0_f32.to_radians());
    app.is_reset_game = false;
}

pub fn update_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_root: Single<&mut Transform, (With<PlayerRoot>, Without<PlayerTurn>)>,
    mut player_turn: Single<&mut Transform, (With<PlayerTurn>, Without<PlayerRoot>)>,
    time: Res<Time>,
    mut app: ResMut<MyApp>,
){
    let ds = time.delta_secs();
    let dir = player_root.forward() * 50.0;
    player_root.translation += dir * ds;

    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let down = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let up = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    if left{
        player_turn.rotation = Quat::from_rotation_z(20.0_f32.to_radians());
        player_root.rotate_local_y(100.0_f32.to_radians() * ds);
    }
    if right{
        player_turn.rotation = Quat::from_rotation_z(-20.0_f32.to_radians());
        player_root.rotate_local_y(-100.0_f32.to_radians() * ds);
    }
    if up {
        player_turn.rotation = Quat::from_rotation_x(5.0_f32.to_radians());
        player_root.translation.y += 20.0 * ds;
    }
    if down && player_root.translation.y > 2.0{
        player_turn.rotation = Quat::from_rotation_x(-5.0_f32.to_radians());
        player_root.translation.y -= 20.0 * ds;
    }
    if !left && !right && !up && !down{
        player_turn.rotation = Quat::from_rotation_x(0.0_f32.to_radians());
        player_turn.rotation = Quat::from_rotation_y(0.0_f32.to_radians());
        player_turn.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
    }

    if keyboard_input.just_pressed(KeyCode::Escape){
        player_root.translation.x = 0.0;
        player_root.translation.y = 2.0;
        player_root.translation.z = 0.0;
        player_root.rotation = Quat::from_rotation_y(0.0_f32.to_radians());
    }

    if player_root.translation.x > value::RESETRANGE || 
        player_root.translation.x < -value::RESETRANGE || 
        player_root.translation.z > value::RESETRANGE || 
        player_root.translation.z < -value::RESETRANGE{
        app.is_reset_game = true;
    }
}

pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<PlayerRoot>, Without<PlayerCameraPos>)>,
    player_root: Single<&GlobalTransform, With<PlayerRoot>>,
    player_camera_pos: Single<&GlobalTransform, With<PlayerCameraPos>>,
){
    camera.translation = player_camera_pos.translation();
    camera.look_at(player_root.translation(), Vec3::Y);
}

pub fn setup_asset(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    /*
    for x in -50..50{
        for z in -50..50{
            commands.spawn((
                Mesh3d(meshes.add(Rectangle::new(9.5, 9.5))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(x as f32 * 10.0, 0.0, z as f32 * 10.0)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                ReleaseResource,
            ));
        }
    }
    */
    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0),
        PlayerRoot,
        ReleaseResource,
    )).with_children(|parent|{
        parent.spawn((
            PointLight{
                shadows_enabled: true,
                color: Color::srgb_u8(255, 255, 255),
                ..default()
            },
            Transform::from_xyz(0.0, 10.0, 0.0),
        ));
        parent.spawn((//カメラ位置
            Transform::from_xyz(0.0, 5.0, 15.0),
            PlayerCameraPos
        ));

        parent.spawn((
           Transform::from_xyz(0.0, 0.0, 0.0),
           PlayerTurn,
        )).with_children(|parent2|{
            parent2.spawn((//本体
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
            parent2.spawn((//左翼
                Mesh3d(meshes.add(Tetrahedron::new(
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, -2.0), 
                    Vec3::new(4.0,0.0, 1.0), 
                    Vec3::new(1.5, 0.25, 0.0)
                ))),
                MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            parent2.spawn((//右翼
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
    });
    // light
    commands.spawn((
        //PointLight {..default() },
        //DirectionalLight::default(),
        Transform::from_xyz(0.0, 3.0, 0.0),
        ReleaseResource,
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 5.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ReleaseResource,
    ));

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
