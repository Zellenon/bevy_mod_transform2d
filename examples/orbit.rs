use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(Transform2dPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit, rotate))
        .run();
}

#[derive(Component)]
struct Orbit {
    point: Vec2,
    speed: f32,
}

#[derive(Component)]
struct Rotate {
    speed: f32,
}

fn setup(mut commands: Commands) {
    // Spawn a sprite at the center with a 2d transform.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.05, 0.05),
                custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
            ..default()
        },
        Transform2d::default(),
        Rotate { speed: 1. },
    ));

    // Spawn a sprite that orbits the center with a 2d transform.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.15, 1.0, 0.2),
                custom_size: Some(Vec2::splat(150.)),
                ..default()
            },
            ..default()
        },
        Transform2d::from_xy(200., 0.),
        Orbit {
            point: Vec2::ZERO,
            speed: 1.5,
        },
        Rotate { speed: -1.2 },
    ));

    // Spawn the camera.
    commands.spawn(Camera2dBundle::default());
}

fn orbit(mut query: Query<(&mut Transform2d, &Orbit)>, time: Res<Time>) {
    for (mut transform, orbit) in &mut query {
        transform.translate_around(orbit.point, orbit.speed * time.delta_seconds());
    }
}

fn rotate(mut query: Query<(&mut Transform2d, &Rotate)>, time: Res<Time>) {
    for (mut transform, rotate) in &mut query {
        transform.rotation += rotate.speed * time.delta_seconds();
    }
}
