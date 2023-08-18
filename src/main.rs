use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Logic Farming Rougelike".into(),
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
        .build(),
    )
    .insert_resource(Money(100.0))
    .add_systems(Startup, setup)
    .add_systems(Update, (character_movement, spawn_pig, pig_lifetime))
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0, 
        min_height: 144.0, 
    };

    commands.spawn(camera);

    let texture: Handle<Image> = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        // TODO: Solve the issue of diagonal movement being faster than in just
        //       cardinal directions
    }
}

// spawn pigs for $10
fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    // exit the loop if the space bar was just pressed to prevent overspawning
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    // get the player transform. single can only be used if one and only one entity will match query
    let player_transform = player.single();

    // check if the player has enough money to buy a pig, subtract $10 if so & spawn pig
    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a pig, remaining money: ${:?}", money.0);

        // get texture for a pig Sprite
        let texture = asset_server.load("pig.png");

        // spawn the pig
        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Pig {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            }
        ));
    }
}

// function to update the timer on the pig's lifetime and return money to the player
fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        // check to see if the pig's lifetime is over. If so, despawn and add money to player
        if pig.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(pig_entity).despawn();

            info!("Pig sold for $15! Current Money: ${:?}", money.0);
        }
    }
}


// create a component for the player to control movement speed
#[derive(Component)]
pub struct Player {
    pub speed: f32
}

// Create a Resource for money that can be earned and spent
// Another option would be to create this as a component on the Player object
#[derive(Resource)]
pub struct Money(pub f32);

// Create a component for the pigs that causes them to expire after a certain period of time
// This will be used to return money to the player after the pig dies (bring home the bacon!)
#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
}