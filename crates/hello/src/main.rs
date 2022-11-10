use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
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
    cmd.spawn()
        .insert(MyPlayer)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0., 2., 0.)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cylinder(1., 0.5))
        .insert(KinematicCharacterController { ..default() })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(MyPlayerHead)
                .insert_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0., 0.75, 0.),
                    // transform: Transform::from_xyz(3., 0.75, 3.).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                });
        });

    cmd.spawn()
        .insert(RigidBody::Fixed)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0., -1., 0.)))
        .insert(Collider::cuboid(100., 1., 100.));
}
