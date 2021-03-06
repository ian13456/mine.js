#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;

use server_common::types::{Block, TypeMap, UV};
use server_utils::json;

pub type Ranges = HashMap<String, UV>;
pub type Blocks = HashMap<u32, Block>;

#[derive(Debug, Clone)]
pub struct Registry {
    pub atlas: image::RgbaImage,
    pub ranges: Ranges,
    pub blocks: Blocks,
    pub uv_side_count: u32,
    pub uv_texture_size: u32,

    name_map: HashMap<String, u32>,
}

impl Registry {
    pub fn new() -> Self {
        let blocks_json: serde_json::Value =
            serde_json::from_reader(File::open("metadata/blocks.json").unwrap()).unwrap();

        let mut base_cache: HashMap<String, serde_json::Value> = HashMap::new();
        let mut texture_map: HashMap<String, image::DynamicImage> = HashMap::new();

        let mut name_map = HashMap::new();

        let mut blocks: Blocks = HashMap::new();

        for (id, value) in blocks_json.as_object().unwrap() {
            // remove first and last characters to remove the ""
            let value_str = value.as_str().unwrap();
            let path = format!("./metadata/blocks/{}", value_str);
            let mut block_json: serde_json::Value =
                serde_json::from_reader(File::open(path).unwrap()).unwrap();

            let base = &block_json["base"];

            let base = match base {
                serde_json::Value::String(base_str) => {
                    Some(base_cache.entry(base_str.to_owned()).or_insert_with(|| {
                        serde_json::from_reader(
                            File::open(format!("./metadata/blocks/{}", base_str).as_str()).unwrap(),
                        )
                        .unwrap()
                    }))
                }
                _ => None,
            }
            .unwrap();

            json::merge(&mut block_json, base, false);

            let textures = &block_json["textures"];
            let mut textures_hash = HashMap::new();

            if !serde_json::Value::is_null(textures) {
                for (side, img_src) in textures.as_object().unwrap().iter() {
                    let img_src_str = img_src.as_str().unwrap();

                    let image = if img_src_str.ends_with(".png") {
                        image::open(&format!("textures/images/{}", img_src_str)).unwrap()
                    } else {
                        // texture data
                        let texture_data: serde_json::Value = serde_json::from_reader(
                            File::open(format!("textures/procedural/{}", img_src_str)).unwrap(),
                        )
                        .unwrap();

                        let color_vec = texture_data["color"].as_array().unwrap().as_slice();

                        let color_r = (color_vec[0].as_f64().unwrap() * 255.0) as u8;
                        let color_g = (color_vec[1].as_f64().unwrap() * 255.0) as u8;
                        let color_b = (color_vec[2].as_f64().unwrap() * 255.0) as u8;

                        let imgbuf = image::ImageBuffer::from_pixel(
                            16,
                            16,
                            image::Rgb([color_r, color_g, color_b]),
                        );

                        image::DynamicImage::ImageRgb8(imgbuf)
                    };

                    texture_map.insert(img_src_str.to_owned(), image);
                    textures_hash.insert(side.to_owned(), img_src_str.to_owned());
                }
            }

            let mut new_block: Block = serde_json::from_value(block_json).unwrap();
            new_block.textures = textures_hash;
            let id = id.parse::<u32>().unwrap();
            name_map.insert(new_block.name.clone(), id);
            blocks.insert(id, new_block);
        }

        // OBTAINED TEXTURE MAP
        let map_size = texture_map.len() as f32;
        let mut shifts = 1;
        let count_per_side = map_size.sqrt().ceil() as u32;
        while 1 << shifts < count_per_side {
            shifts += 1;
        }
        let count_per_side = 1 << shifts;
        let texture_dim = 64;
        let atlas_width = count_per_side * texture_dim;
        let atlas_height = count_per_side * texture_dim;

        let mut atlas: image::RgbaImage = image::ImageBuffer::new(atlas_width, atlas_height);

        let mut ranges = HashMap::new();

        let mut row = 0;
        let mut col = 0;

        let mut texture_map_vec: Vec<_> = texture_map.into_iter().collect();
        texture_map_vec.sort_by(|x, y| x.0.cmp(&y.0));

        for (key, image) in texture_map_vec {
            if col >= count_per_side {
                col = 0;
                row += 1;
            }

            let start_x = col * texture_dim;
            let start_y = row * texture_dim;

            let resized = image::imageops::resize(
                &image,
                texture_dim,
                texture_dim,
                image::imageops::FilterType::CatmullRom,
            );

            image::imageops::overlay(&mut atlas, &resized, start_x, start_y);

            let f_start_x = start_x as f32;
            let f_start_y = start_y as f32;

            let f_atlas_width = atlas_width as f32;
            let f_atlas_height = atlas_height as f32;

            let start_u = f_start_x / f_atlas_width;
            let end_u = (f_start_x + texture_dim as f32) / f_atlas_width;
            let start_v = 1.0 - f_start_y / f_atlas_height;
            let end_v = 1.0 - (f_start_y + texture_dim as f32) / f_atlas_height;

            let (start_u, start_v, end_u, end_v) =
                fix_texture_bleeding((start_u, start_v, end_u, end_v), texture_dim);

            let uv = UV {
                start_u,
                end_u,
                start_v,
                end_v,
            };

            ranges.insert(key, uv);

            col += 1;
        }

        atlas.save("textures/atlas.png").unwrap();

        Self {
            atlas,
            ranges,
            blocks,
            uv_texture_size: texture_dim,
            uv_side_count: count_per_side,
            name_map,
        }
    }

    #[inline]
    pub fn get_transparency_by_id(&self, id: u32) -> bool {
        self.get_block_by_id(id).is_transparent
    }

    #[inline]
    pub fn get_transparency_by_name(&self, name: &str) -> bool {
        self.get_block_by_name(name).is_transparent
    }

    #[inline]
    pub fn get_fluiditiy_by_id(&self, id: u32) -> bool {
        self.get_block_by_id(id).is_fluid
    }

    #[inline]
    pub fn get_fluiditiy_by_name(&self, name: &str) -> bool {
        self.get_block_by_name(name).is_fluid
    }

    #[inline]
    pub fn get_solidity_by_id(&self, id: u32) -> bool {
        self.get_block_by_id(id).is_solid
    }

    #[inline]
    pub fn get_solidity_by_name(&self, name: &str) -> bool {
        self.get_block_by_name(name).is_solid
    }

    #[inline]
    pub fn get_emptiness_by_id(&self, id: u32) -> bool {
        self.get_block_by_id(id).is_empty
    }

    #[inline]
    pub fn get_emptiness_by_name(&self, name: &str) -> bool {
        self.get_block_by_name(name).is_empty
    }

    #[inline]
    pub fn get_texture_by_id(&self, id: u32) -> &HashMap<String, String> {
        &self.get_block_by_id(id).textures
    }

    #[inline]
    pub fn get_texture_by_name(&self, name: &str) -> &HashMap<String, String> {
        &self.get_block_by_name(name).textures
    }

    #[inline]
    pub fn get_uv_by_id(&self, id: u32) -> HashMap<String, &UV> {
        self.get_uv_map(self.get_block_by_id(id))
    }

    #[inline]
    pub fn get_uv_by_name(&self, name: &str) -> HashMap<String, &UV> {
        self.get_uv_map(self.get_block_by_name(name))
    }

    #[inline]
    pub fn is_air(&self, id: u32) -> bool {
        self.get_block_by_id(id).name == "Air"
    }

    #[inline]
    pub fn is_plant(&self, id: u32) -> bool {
        self.get_block_by_id(id).is_plant
    }

    #[inline]
    pub fn is_plantable(&self, id: u32) -> bool {
        self.get_block_by_id(id).is_plantable
    }

    #[inline]
    pub fn get_block_by_id(&self, id: u32) -> &Block {
        self.blocks
            .get(&id)
            .unwrap_or_else(|| panic!("Block id not found: {}", id))
    }

    #[inline]
    pub fn get_block_by_name(&self, name: &str) -> &Block {
        let &id = self
            .name_map
            .get(name)
            .unwrap_or_else(|| panic!("Block name not found: {}", name));
        self.get_block_by_id(id)
    }

    pub fn get_id_by_name(&self, name: &str) -> &u32 {
        self.name_map
            .get(name)
            .unwrap_or_else(|| panic!("Type name not found: {}", name))
    }

    pub fn get_uv_map(&self, block: &Block) -> HashMap<String, &UV> {
        let mut uv_map = HashMap::new();

        for source in block.textures.values().into_iter() {
            let uv = self
                .ranges
                .get(source)
                .unwrap_or_else(|| panic!("UV range not found: {}", source));

            uv_map.insert(source.to_owned(), uv);
        }

        uv_map
    }

    pub fn get_type_map(&self, blocks: Vec<&str>) -> TypeMap {
        let mut type_map = HashMap::new();

        for block in blocks {
            let &id = self
                .name_map
                .get(block)
                .unwrap_or_else(|| panic!("Block name not found: {}", block));

            type_map.insert(block.to_owned(), id);
        }

        type_map
    }

    pub fn get_passable_solids(&self) -> Vec<u32> {
        self.blocks
            .iter()
            .filter(|&(_, b)| !b.is_solid && (b.is_block || b.is_plant))
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn has_type(&self, id: u32) -> bool {
        self.blocks.contains_key(&id)
    }
}

pub fn get_texture_type(texture: &HashMap<String, String>) -> &str {
    let len = texture.len();

    if len == 1 {
        "mat1"
    } else if len == 3 {
        "mat3"
    } else if len == 6 {
        "mat6"
    } else {
        "x"
    }
}

fn fix_texture_bleeding(
    (start_u, start_v, end_u, end_v): (f32, f32, f32, f32),
    texture_size: u32,
) -> (f32, f32, f32, f32) {
    let offset = 0.1 / texture_size as f32;
    (
        start_u + offset,
        start_v - offset,
        end_u - offset,
        end_v + offset,
    )
}
