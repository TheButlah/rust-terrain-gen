mod height_map;

use ndarray::parallel::prelude::*;

use height_map::{Height, HeightMap};

type Rgb = image::Rgb<u8>;
type TerrainMap = ndarray::Array2<TerrainType>;

#[derive(Copy, Clone)]
pub enum TerrainType {
    Water,
    Grass,
    Mountain,
    Snow,
}
impl TerrainType {
    pub const fn as_color(&self) -> &'static [u8] {
        match self {
            Self::Water => &[0, 0, 255],
            Self::Grass | _ => &[0, 255, 0],
        }
    }
}

pub struct Terrain {
    height_map: HeightMap,
    terrain_map: TerrainMap,
}
impl Terrain {
    pub fn new(height_map: HeightMap) -> Self {
        let shape = height_map.shape();
        let shape = [shape[0], shape[1]];
        let terrain_map: Vec<TerrainType> = height_map
            .par_iter()
            .map(|height| {
                if *height < 0.0 {
                    TerrainType::Water
                } else {
                    TerrainType::Grass
                }
            })
            .collect();
        let terrain_map = TerrainMap::from_shape_vec(shape, terrain_map).unwrap();
        Self {
            terrain_map,
            height_map,
        }
    }

    fn to_img(&self) -> image::RgbImage {
        let buf: Vec<u8> = self
            .terrain_map
            .iter()
            .map(|t_type| t_type.as_color())
            .flatten()
            .copied()
            .collect();

        let shape = self.terrain_map.shape();
        image::RgbImage::from_raw(shape[0] as u32, shape[1] as u32, buf)
            .expect("Vector should have been large enough, but wasn't")
    }
}
