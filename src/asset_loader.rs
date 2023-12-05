use bevy::prelude::*;

#[derive(Debug,Default, Resource,)]
pub struct SceneAssets{
    pub asteroid:   Handle<Scene>,
    pub spaceship:  Handle<Scene>,
    pub missile:    Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: ResMut<AssetServer>){
    *scene_assets = SceneAssets{
        asteroid: asset_server.load("asteroid.glb#Scene0"),
        spaceship: asset_server.load("spaceship.glb#Scene0"),
        missile: asset_server.load("missile.glb#Scene0")
    }
}