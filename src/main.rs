use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}, transform,
};

use rand::Rng;


#[derive(Component)]
struct Ball_Parameters {
    attractions: Vec<Vec<f32>>,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct Speed {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct BallKind{kind: u8}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<Window>>,
) {
    commands.spawn(Camera2dBundle::default());

    let number_of_ball_kinds = 5;

    let number_of_balls_per_kind = 50;

    let window = window_query.get_single().unwrap();

    for i in 0..number_of_ball_kinds {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / number_of_ball_kinds as f32, 0.95, 0.7);

        for _j in 0..number_of_balls_per_kind{
        let mut rang = rand::thread_rng();

        let x:f32 = rang.gen::<f32>();
        let y:f32 = rang.gen::<f32>();

        commands.spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 5.0 })),
            material: materials.add(color),
            transform: Transform::from_xyz(
                x*window.width()-(window.width()/2.0),
                y*window.height()-(window.height()/2.0),
                0.0,
            ),
            ..default()
        },Position{x,y},Speed{x:0.0,y:0.0},BallKind{kind: i}),);
        }
    }
}

fn update_balls(
    mut query: Query<(&mut Position, &mut Speed, &BallKind, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<Window>>,
) {
    for (mut pos, vel, kind, transform) in &mut query {
        
    }}



/*
let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
*/