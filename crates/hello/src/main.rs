use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(my_setup)
        .add_system(grab_mouse)
        .add_system(my_player_control)
        .run();
}

// Marker, entity is the player
#[derive(Component)]
struct MyPlayer;

// Marker, entity is the head of the player entity
#[derive(Component)]
struct MyPlayerHead;

fn my_setup(mut cmd: Commands) {
    cmd.spawn((
        MyPlayer,
        TransformBundle::from(Transform::from_xyz(0., 2., 0.)),
        RigidBody::KinematicPositionBased,
        Collider::cylinder(1., 0.5),
        KinematicCharacterController { ..default() },
    ))
    .with_children(|parent| {
        parent.spawn((
            MyPlayerHead,
            Camera3dBundle {
                transform: Transform::from_xyz(0., 0.75, 0.),
                ..default()
            },
        ));
    });

    cmd.spawn((
        TransformBundle::from(Transform::from_xyz(0., -1e-5, 0.)),
        RigidBody::Fixed,
        Collider::cuboid(1e5, 1e-5, 1e5),
    ));
}

fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let win = windows.primary_mut();

    if mouse.just_pressed(MouseButton::Left) {
        win.set_cursor_visibility(false);
        win.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
    }

    if key.pressed(KeyCode::Escape) {
        win.set_cursor_visibility(true);
        win.set_cursor_grab_mode(bevy::window::CursorGrabMode::None);
    }
}

fn my_player_control(
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    key: Res<Input<KeyCode>>,
) {
    let view_angle_delta: Vec2 = mouse_motion.iter().map(|ev| ev.delta).sum();
    let wish_dir: Vec3 = [
        (KeyCode::W, Vec3::X),
        (KeyCode::S, -Vec3::X),
        (KeyCode::A, Vec3::Z),
        (KeyCode::D, -Vec3::Z),
    ]
    .into_iter()
    .filter_map(|(k, v)| key.pressed(k).then_some(v))
    .sum();
}
