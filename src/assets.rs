use bevy::prelude::*;
use crate::constants::*;
use crate::components::*;
pub struct AssetsPlugin;

const VEHICLE_TILEMAP: &str = "kenney/pixel_vehicle_pack/spritesheet_complete.png";

impl Plugin for AssetsPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut commands: Commands,
                   asset_server: Res<AssetServer>,
               mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let vehicle_atlas_handle = texture_atlases.add(
        TextureAtlas::from_grid(asset_server.load(VEHICLE_TILEMAP),
                                Vec2::new(TILE_WIDTH, TILE_HEIGHT),
                                16, 7,
                                Option::from(Vec2::new(1.0, 1.0)),
                                None));
    commands.insert_resource(SpriteSheetHandles{
        cars: vehicle_atlas_handle
    });
}