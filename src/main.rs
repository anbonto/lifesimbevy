use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}, transform,
};

use rand::Rng;


#[derive(Component)]
struct Ball_Parameters {
    attractions: Vec<Vec<f32>>,
}

fn random_attraction_parameters(number_of_ball_kinds: u8) -> Vec<Vec<f32>> {
    let mut parameters: Vec<Vec<f32>>  = Vec::new();
    for i in 0..number_of_ball_kinds {
        parameters.push(Vec::new());
        for j in 0..number_of_ball_kinds {
            let mut rng = rand::thread_rng();
            let randbool: bool = rng.gen();
            if i != j {
                parameters[i as usize].push(rng.gen::<f32>() * (if randbool {1.0} else {-1.0}));
        }else{parameters[i as usize].push(rng.gen::<f32>() * (if randbool {1.0} else {-1.0})/4.0);}
        }
    }
    parameters
}


#[derive(Component, Clone)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct Speed {
    x: f32,
    y: f32,
}
#[derive(Component, Clone)]
struct BallKind{kind: u8}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_balls)
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

    commands.spawn((Ball_Parameters{attractions: random_attraction_parameters(number_of_ball_kinds.clone())},),);

    for i in 0..number_of_ball_kinds {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / number_of_ball_kinds as f32, 0.95, 0.7);

        for _j in 0..number_of_balls_per_kind{
        let mut rang = rand::thread_rng();

        let x:f32 = rang.gen::<f32>();
        let y:f32 = rang.gen::<f32>();

        commands.spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 3.0 })),
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
    query2: Query<&Ball_Parameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<Window>>,
) {
    let window = window_query.get_single().unwrap();
    let parameters = query2.get_single().unwrap();
    let mut pos_copy : Vec<(Position, BallKind)> = Vec::new();

    for (mut pos, vel, kind, mut transform) in &mut query {
        pos.x += vel.x;
        pos.y += vel.y;
        transform.translation.x = pos.x*window.width()-(window.width()/2.0);
        transform.translation.y = pos.y*window.height()-(window.height()/2.0);
        pos_copy.push((pos.clone(), kind.clone()));
    }

    for (pos, mut vel, kind, _transform) in &mut query {
        let mut accelerationx = 0.0;
        let mut accelerationy = 0.0;
        for i in 0..pos_copy.len() {
            if !(pos_copy[i].0.x == pos.x && pos_copy[i].0.y == pos.y) {
            let mut distancex = (pos_copy[i].0.x - pos.x).abs();
            let mut distancey = (pos_copy[i].0.y - pos.y).abs();


            let mut higherx = if pos_copy[i].0.x>pos.x {1.0} else {-1.0};
            let mut highery = if pos_copy[i].0.y>pos.y {1.0} else {-1.0};

            //edge conditions
            if distancex > 0.5{
                distancex = 1.0 - distancex;
                higherx *= -1.0;
            }

            if distancey > 0.5{
                distancey = 1.0 - distancey;
                highery *= -1.0;
            } 
            let distance = ((distancex).powi(2) + (distancey).powi(2)).sqrt();
            if distance > 0.04{}
            else if distance > 0.02{
                accelerationx +=  parameters.attractions[kind.kind as usize][pos_copy[i].1.kind as usize]*0.00002 * (1.0-(distance*25.0)) * higherx* (1.0-distancex*25.0);
                accelerationy +=  parameters.attractions[kind.kind as usize][pos_copy[i].1.kind as usize]*0.00002 * (1.0-(distance*25.0)) * highery * (1.0-distancey*25.0);
            }
            
            else if distance > 0.004{
                accelerationx +=  parameters.attractions[kind.kind as usize][pos_copy[i].1.kind as usize]*0.00002 * ((distance-0.008)*31.25) * higherx * (1.0-distancex*50.0);
                accelerationy +=  parameters.attractions[kind.kind as usize][pos_copy[i].1.kind as usize]*0.00002 * ((distance-0.008)*31.25) * highery * (1.0-distancey*50.0);
            }

            else{
                accelerationx +=  -0.00002 * (1.0-(distance*250.0)) * 25.0 * higherx * (1.0-distancex*250.0);
                accelerationy +=  -0.00002 * (1.0-(distance*250.0)) * 25.0 * highery * (1.0-distancey*250.0);
            }
        }
        }

        vel.x += accelerationx;
        vel.y += accelerationy;
        vel.x *= 0.95;
        vel.y *= 0.95;


    }
}



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