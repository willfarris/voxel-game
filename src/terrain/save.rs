use std::collections::HashMap;

use json::{JsonValue, object};

use super::{Terrain, ChunkIndex, chunk::{Chunk, BlockDataArray, CHUNK_WIDTH, CHUNK_HEIGHT}};

pub(crate) fn save_chunk_data_to_json(data: &BlockDataArray) -> JsonValue {
    let mut json_vec = Vec::with_capacity(CHUNK_WIDTH);
    for row in data.iter() {
        let mut json_row = Vec::with_capacity(CHUNK_HEIGHT);
        for column in row.iter() {
            let json_column = json::from(column.as_slice());
            json_row.push(json_column);
        };
        json_vec.push(json_row);
    }
    json::from(json_vec)
}

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