use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use cgmath::Vector3;
use json::{object, JsonValue};

use crate::{
    entity::EntityTrait,
    graphics::{resources::GLResources, skybox::Skybox},
    player::Player,
    terrain::{generation::TerrainGenConfig, Terrain},
};

use super::{Engine, PlayState};

impl Engine {
    pub fn load_from_save(save_path: &str) -> Self {
        let mut save_file_path = std::path::PathBuf::new();
        save_file_path.push(save_path);
        save_file_path.push("savestate.json");
        let save_file = std::fs::read_to_string(save_file_path).unwrap();
        let save_json = json::parse(&save_file).unwrap();

        let player_json = &save_json["player"];
        let player_position_json = &player_json["position"];
        let player_position = Vector3::new(
            player_position_json[0].as_f32().unwrap(),
            player_position_json[1].as_f32().unwrap(),
            player_position_json[2].as_f32().unwrap(),
        );
        let player_orientation_json = &player_json["orientation"];
        let player_direction = Vector3::new(
            player_orientation_json[0].as_f32().unwrap(),
            player_orientation_json[1].as_f32().unwrap(),
            player_orientation_json[2].as_f32().unwrap(),
        );
        let player = Box::new(Player::new(player_position, player_direction));

        let terrain_json = &save_json["terrain"];
        let terrain = Terrain::load_from_json(terrain_json);

        let entities: Vec<Box<dyn EntityTrait>> = Vec::new();
        let mut terrain_config: TerrainGenConfig = TerrainGenConfig::default();
        terrain_config.load_features(include_str!("../../assets/features/world_features.json"));

        Self {
            player: Arc::new(RwLock::new(player)),
            terrain: Arc::new(RwLock::new(terrain)),
            entities,
            skybox: Skybox,

            elapsed_time: Duration::ZERO,
            last_update: Instant::now(),

            play_state: PlayState::Paused,
            input_queue: Vec::new(),
            terrain_config: Arc::new(RwLock::new(terrain_config)),

            width: 0,
            height: 0,
            render_distance: 8,
            gl_resources: Arc::new(RwLock::new(GLResources::new())),
        }
    }

    pub fn save_to_file(&mut self, save_path: &str) {
        let save_path = std::path::Path::new(save_path);
        if !save_path.exists() {
            std::fs::create_dir_all(save_path).unwrap();
        }
        let mut save_file_path = std::path::PathBuf::new();
        save_file_path.push(save_path);
        save_file_path.push("savestate.json");
        println!("Saving to {:?}", save_file_path.as_path());

        let mut save_json = JsonValue::new_object();
        {
            let player = self.player.write().unwrap();
            let player_json = object! {
                "position" : [player.position.x, player.position.y, player.position.z],
                "orientation" : [player.camera.forward.x, player.camera.forward.y, player.camera.forward.z],
            };
            save_json.insert("player", player_json).unwrap();
        }
        {
            let terrain = self.terrain.write().unwrap();
            let terrain_json = terrain.to_json();
            save_json.insert("terrain", terrain_json).unwrap();
        }

        std::fs::write(save_file_path, save_json.dump()).unwrap();
    }
}
