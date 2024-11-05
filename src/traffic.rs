use bevy::prelude::*;
use crate::constants::*;
use crate::components::*;

pub struct TrafficPlugin;


impl Plugin for TrafficPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(Startup, spawn_traffic)
            .add_systems(Update, traffic_management)
            .add_systems(Update, movement)
            ;
    }
}

fn traffic_management(mut commands: Commands, mut traffic_query: Query<(&mut Traffic)>, time: Res<Time>, 
sprite_sheets: Res<SpriteSheetHandles>) {
    for(mut traffic) in traffic_query.iter_mut() {
        traffic.time_since_last_car += time.delta_seconds();
        if traffic.time_since_last_car > traffic.next_car_at {
            commands.spawn((SpriteSheetBundle {
                texture_atlas: sprite_sheets.cars.clone(),
                sprite: TextureAtlasSprite::new(CAR_SPRITE_INDEX),
                transform: Transform {
                    translation: Vec3 { x: SCREEN_LEFT, y: CAR_Y, z: 0f32 },
                    scale: Vec3 {x: CAR_SCALE, y: CAR_SCALE, z: CAR_SCALE},
                    ..default()
                },
                ..default()
               },
               Car{speed: CAR_SPEED}
            ));
            traffic.time_since_last_car -= traffic.next_car_at;
        }
    }
}


fn movement(mut commands: Commands, mut traffic_query: Query<(Entity, &Car, &mut Transform)>,
    time: Res<Time>) {
    for (entity, car, mut transform) in traffic_query.iter_mut() {
        transform.translation.x += car.speed * time.delta_seconds();
        if transform.translation.x > SCREEN_RIGHT {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_traffic(mut commands: Commands) {
    let traffic = Traffic{
        time_since_last_car: 0f32,
        next_car_at: CAR_INTERVAL,
    };

    commands.spawn(traffic);
}