

// JSON Defs for loading atlas

use std::fs;
use std::fs::File;
use std::io::Read;
use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};
use crate::game_object::graphics::SpriteID;
use crate::renderer::sprite::SpriteVertex;

#[derive(Deserialize,Serialize)]
pub struct AtlasVector2 {
    x: u64,
    y: u64
}
#[derive(Deserialize,Serialize)]
pub struct AtlasSpriteSize {
    width: u64,
    height: u64
}
#[derive(Deserialize,Serialize)]
pub struct GenericAtlasData {
    width: u64,
    height: u64,
    spriteCount: u64
}
#[derive(Deserialize,Serialize)]
pub struct AtlasSpriteTrimInfo {
    x: u64,
    y: u64,
    width: u64,
    height: u64
}
#[derive(Deserialize,Serialize)]
pub struct AtlasSprite {
    nameId: String,
    origin: AtlasVector2,
    position: AtlasVector2,
    sourceSize: AtlasSpriteSize,
    padding: u64,
    trimmed: bool,
    trimRec: AtlasSpriteTrimInfo
}

pub struct ParsedAtlasSprite {
    pub origin: AtlasVector2,
    pub position: AtlasVector2,
    pub sourceSize: AtlasSpriteSize,
    pub padding: u64,
    pub trimmed: bool,
    pub trimRec: AtlasSpriteTrimInfo
}

#[derive(Deserialize,Serialize)]
pub struct AtlasDescriptor {
    atlas: GenericAtlasData,
    sprites: Vec<AtlasSprite>
}
//
pub struct SpriteAtlas {
   pub width: u64, pub height: u64, pub atlas: Vec<u8>, sprites: HashMap<SpriteID,ParsedAtlasSprite>
}

pub(crate) fn get_file_as_byte_vector(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

impl SpriteAtlas {
    pub fn new(
        atlas_file: &str,
        atlas_descriptor_file: &str
    ) -> Self {
        let atlas_descriptor_file_contents = fs::read_to_string(atlas_descriptor_file).unwrap();
        let atlas_descriptor: AtlasDescriptor = serde_json::from_str(&atlas_descriptor_file_contents).unwrap();

        let mut sprites = HashMap::new();

        for sprite in atlas_descriptor.sprites {
            sprites.insert(sprite.nameId,ParsedAtlasSprite {
                origin: sprite.origin,
                position: sprite.position,
                sourceSize: sprite.sourceSize,
                padding: sprite.padding,
                trimmed: sprite.trimmed,
                trimRec: sprite.trimRec,
            });
        }

        Self {
            width: atlas_descriptor.atlas.width,
            height: atlas_descriptor.atlas.height,
            atlas: get_file_as_byte_vector(atlas_file),
            sprites: sprites
        }
    }

    pub fn get_sprite_from_atlas(
        &self,
        atlas_sprite_position: &AtlasVector2,
        atlas_sprite_size: &AtlasSpriteSize,
        local_sprite_position: [f32; 2],
        local_sprite_scale: [f32; 2],
        flip_h: bool,
        flip_v: bool,
    ) -> ([SpriteVertex; 4], [u16; 6]) {
        let (u1, u2) = if flip_h {
            let x = (atlas_sprite_position.x + atlas_sprite_size.width) as f32 / self.width as f32;
            let y = atlas_sprite_position.x as f32 / self.width as f32;
            (x, y)
        } else {
            let x = atlas_sprite_position.x as f32 / self.width as f32;
            let y = (atlas_sprite_position.x + atlas_sprite_size.width) as f32 / self.width as f32;
            (x, y)
        };

        let (v1, v2) = if flip_v {
            let x = atlas_sprite_position.y as f32 / self.height as f32;
            let y = (atlas_sprite_position.y + atlas_sprite_size.height) as f32 / self.height as f32;
            (x, y)
        } else {
            let x = (atlas_sprite_position.y + atlas_sprite_size.height) as f32 / self.height as f32;
            let y = atlas_sprite_position.y as f32 / self.height as f32;
            (x, y)
        };

        let vertices = [
            SpriteVertex {
                position: [local_sprite_position[0], local_sprite_position[1], 0.0],
                tex_coords: [u1, v1],
            }, // Bottom-Left
            SpriteVertex {
                position: [
                    local_sprite_position[0] + local_sprite_scale[0],
                    local_sprite_position[1],
                    0.0,
                ],
                tex_coords: [u2, v1],
            }, // Bottom-Right
            SpriteVertex {
                position: [
                    local_sprite_position[0] + local_sprite_scale[0],
                    local_sprite_position[1] + local_sprite_scale[1],
                    0.0,
                ],
                tex_coords: [u2, v2],
            }, // Top-Right
            SpriteVertex {
                position: [
                    local_sprite_position[0],
                    local_sprite_position[1] + local_sprite_scale[1],
                    0.0,
                ],
                tex_coords: [u1, v2],
            }, // Top-Left
        ];
        let indices = [0, 1, 2, 0, 2, 3];
        (vertices, indices)
    }

    pub fn lookup_sprite_data_from_descriptor(&self,id: &SpriteID) -> &ParsedAtlasSprite {
        let sprite = self.sprites.get(id).unwrap();
        return sprite;
    }
}