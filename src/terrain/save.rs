use std::collections::HashMap;

use json::{JsonValue, object};

use super::{Terrain, ChunkIndex, chunk::Chunk};

impl Terrain {
    pub fn load_from_json(terrain_json: &JsonValue) -> Self {
        let mut chunks = HashMap::new();
        let chunks_json = &terrain_json["chunks"];
        for (key, chunk_data) in chunks_json.entries() {
            let coords: Vec<isize> = key.split('_').map(|s| {
                s.parse().unwrap()
            }).collect();
            let chunk_index = ChunkIndex::new(coords[0] as isize, coords[1] as isize);
            let chunk = Chunk::from_json_array(chunk_data);
            chunks.insert(chunk_index, chunk);
        }
        Self {
            player_visible: Vec::new(),
            chunks,
            placement_queue: HashMap::new(),
        }
    }

    pub fn to_json(&self) -> JsonValue {

        let mut chunks = JsonValue::new_object();
        for (chunk_index, chunk) in self.chunks.iter() {
            let key = format!("{}_{}", chunk_index.x, chunk_index.y);
            let chunk_data = chunk.to_json_array();
            chunks.insert(key.as_str(), chunk_data).unwrap();
        }

        object!{
            "chunks" : chunks,
        }
    }
}