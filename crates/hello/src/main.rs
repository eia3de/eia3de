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
        RigidBody::Fixed,
        TransformBundle::from(Transform::from_xyz(0., -1e-5, 0.)),
        Collider::cuboid(1e5, 1e-5, 1e5),
    ));
}
