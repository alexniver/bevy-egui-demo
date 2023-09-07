use bevy::{prelude::*, window::close_on_esc};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Camera"),
    ));

    // light
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 8000.0,
                ..default()
            },
            transform: Transform::from_xyz(20.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Light Sun"),
    ));

    // plant
    commands.spawn(PbrBundle {
        mesh: meshs.add(shape::Plane::default().into()),
        material: materials.add(Color::DARK_GREEN.into()),
        transform: Transform::default().with_scale(Vec3::new(20.0, 1.0, 20.0)),
        ..default()
    });

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::Cube::default().into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(2.0, 0.5, 2.0),
            ..default()
        },
        Name::new("block"),
    ));
}
