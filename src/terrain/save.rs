use std::{collections::HashMap, sync::{Arc, RwLock}};

use json::{object, JsonValue};
use lockfree::queue::Queue;

use super::{
    chunk::{BlockDataArray, Chunk, CHUNK_HEIGHT, CHUNK_WIDTH}, generation::TerrainGenConfig, ChunkIndex, Terrain
};

pub(crate) fn save_chunk_data_to_json<T: Clone + std::convert::Into<json::JsonValue>>(
    data: &BlockDataArray<T>,
) -> JsonValue {
    let mut json_vec = Vec::with_capacity(CHUNK_WIDTH);
    for row in data.iter() {
        let mut json_row = Vec::with_capacity(CHUNK_HEIGHT);
        for column in row.iter() {
            let json_column = json::from(column.as_slice());
            json_row.push(json_column);
        }
        json_vec.push(json_row);
    }
    json::from(json_vec)
}

impl Terrain {
    pub fn load_from_json(terrain_json: &JsonValue, config: TerrainGenConfig) -> Self {
        let mut chunks = HashMap::new();
        let chunks_json = &terrain_json["chunks"];
        for (key, chunk_data) in chunks_json.entries() {
            let coords: Vec<isize> = key.split('_').map(|s| s.parse().unwrap()).collect();
            let chunk_index = ChunkIndex::new(coords[0], coords[1]);
            let chunk = Arc::new(RwLock::new(Chunk::from_json_array(chunk_data)));
            chunks.insert(chunk_index, chunk);
        }
        Self {
            chunks: [HashMap::new(), chunks],
            block_placement_queue: HashMap::new(),
            chunk_generation_queue: Arc::new(Queue::new()),
            event_queue: Vec::new(),
            config
        }
    }

    pub fn to_json(&self) -> JsonValue {
        let mut chunks = JsonValue::new_object();
        let priority_levels = self.chunks.len();
        for p in 0..priority_levels {
            for (chunk_index, chunk) in self.chunks[p].iter() {
                let key = format!("{}_{}", chunk_index.x, chunk_index.y);
                let chunk_data = chunk.read().unwrap().to_json_array();
                chunks.insert(key.as_str(), chunk_data).unwrap();
            }
        }

        object! {
            "chunks" : chunks,
        }
    }
}
