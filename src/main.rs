use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide, time::common_conditions::on_timer};
use rand::{seq::SliceRandom, thread_rng, Rng};

const PLAYER_INITIAL_POSITION: Vec2 = Vec2::new(0., -350.);

const PLAYER_SIZE: Vec2 = Vec2::new(50., 50.);
const FRUITS_SIZE: Vec2 = Vec2::new(30., 30.);

const FRUITS_SPAWN_POSITION_Y: f32 = 350.;

const PLAYER_SPEED: f32 = 400.0;
const FRUITS_SPEED: f32 = 300.0;

const PLAYER_CLAMP: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Game>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                fruits_catch,
                move_player,
                move_fruits,
                create_fruits.run_if(on_timer(Duration::from_secs_f32(2.))),
            ),
        )
        .run();
}

#[derive(Debug, Default, Resource)]
struct Game {
    score: usize,
}

#[derive(Component)]
struct Player;

#[derive(Component, Clone, Copy)]
enum Fruits {
    Banana,
    Budou,
}

#[derive(Bundle)]
struct FruitsBundle {
    sprite_bundle: SpriteBundle,
    fruits: Fruits,
}

impl Fruits {
    const FRUITS_LIST: [Fruits; 2] = [Fruits::Budou, Fruits::Banana];

    fn random() -> Fruits {
        *Self::FRUITS_LIST.choose(&mut thread_rng()).unwrap()
    }
}

impl FruitsBundle {
    fn new(fruits: Fruits, texture: Handle<Image>) -> FruitsBundle {
        FruitsBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: random_position(),
                    scale: FRUITS_SIZE.extend(0.0),
                    ..default()
                },
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.5, 1.5)),
                    ..default()
                },
                ..default()
            },
            fruits,
        }
    }
}

fn random_position() -> Vec3 {
    Vec3::new(
        thread_rng().gen_range(-380.0..380.0),
        FRUITS_SPAWN_POSITION_Y,
        0.0,
    )
}

#[derive(Resource)]
struct ImageResouce {
    banana: Handle<Image>,
    budou: Handle<Image>,
}

impl ImageResouce {
    fn get(&self, fruits: Fruits) -> Handle<Image> {
        match fruits {
            Fruits::Banana => self.banana.clone(),
            Fruits::Budou => self.budou.clone(),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(ImageResouce {
        banana: asset_server.load("images/banana.png"),
        budou: asset_server.load("images/budou.png"),
    });

    commands.spawn((
        Player,
        SpriteBundle {
            transform: Transform {
                translation: PLAYER_INITIAL_POSITION.extend(0.0),
                scale: PLAYER_SIZE.extend(0.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn fruits_catch(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    fruits: Query<(Entity, &Transform), With<Fruits>>,
    mut game: ResMut<Game>,
) {
    let player = player.single();
    let player_pos = player.translation;
    let player_size = player.scale.xy();

    for (entity, transform) in &fruits {
        let collision = collide(
            player_pos,
            player_size,
            transform.translation,
            transform.scale.xy(),
        );
        if collision.is_some() {
            game.score += 1;
            info!("{:?}", game);
            commands.entity(entity).despawn();
        }
    }
}

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    key: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = player.single_mut();

    let mut direction: f32 = 0.0;

    if key.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction += 1.0
    }
    if key.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction -= 1.0
    }
    if direction == 0.0 {
        return;
    }

    let new_pos = transform.translation.x + PLAYER_SPEED * direction * time.delta_seconds();
    transform.translation.x = new_pos.clamp(-PLAYER_CLAMP, PLAYER_CLAMP);
}

fn move_fruits(
    mut commands: Commands,
    mut fruits_query: Query<(Entity, &mut Transform), With<Fruits>>,
    time: Res<Time>,
) {
    for (entity, mut transform) in &mut fruits_query {
        transform.translation.y -= FRUITS_SPEED * time.delta_seconds();
        if transform.translation.y < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn create_fruits(mut commands: Commands, images: Res<ImageResouce>) {
    let fruits = Fruits::random();
    let image = images.get(fruits);

    commands.spawn(FruitsBundle::new(fruits, image));
}
