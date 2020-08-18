use super::height_map::{Height, HeightMap};

type TerrainMap = ndarray::Array2<TerrainType>;

#[derive(Copy, Clone)]
pub enum TerrainType {
    Water,
    Grass(Height),
    Mountain,
    Snow,
}
impl TerrainType {
    pub fn to_color(&self) -> [u8; 3] {
        match *self {
            Self::Water => [0, 0, 255],
            Self::Mountain => [223, 137, 64],
            Self::Grass(height) => {
                assert!(height >= 0.0 && height <= 1.0);
                let g = height * 100.0 + 155 as f32;
                let g = num::clamp(g as u8, 0, 255);
                [0, g, 0]
            }
            Self::Snow => [255, 255, 255],
        }
    }
}

pub struct Terrain {
    height_map: HeightMap,
    terrain_map: TerrainMap,
}
impl Terrain {
    pub fn shape(&self) -> (usize, usize) {
        if let [rows, cols, ..] = self.terrain_map.shape() {
            return (*rows, *cols);
        }
        unreachable!()
    }

    pub fn to_rgb(&self) -> Vec<u8> {
        let buf: Vec<[u8; 3]> =
            self.terrain_map.iter().map(TerrainType::to_color).collect();

        let buf: Vec<u8> = buf.iter().flatten().copied().collect();
        buf
    }

    pub fn to_rgba(&self) -> Vec<u8> {
        let buf: Vec<[u8; 4]> = self
            .terrain_map
            .iter()
            .map(|t| {
                let rgb = t.to_color();
                [rgb[0], rgb[1], rgb[2], 255]
            })
            .collect();

        let buf: Vec<u8> = buf.iter().flatten().copied().collect();
        buf
    }
}
impl From<HeightMap> for Terrain {
    fn from(height_map: HeightMap) -> Self {
        let water = ..0.0 as Height;
        let grass = water.end..0.5;
        let mountain = grass.end..0.65;
        let snow = mountain.end..;

        let shape = height_map.shape();
        let shape = [shape[0], shape[1]];
        let terrain_map: Vec<TerrainType> = height_map
            .iter()
            .map(|h| {
                if water.contains(h) {
                    TerrainType::Water
                } else if grass.contains(h) {
                    TerrainType::Grass(h / (grass.end - grass.start))
                } else if mountain.contains(h) {
                    TerrainType::Mountain
                } else if snow.contains(h) {
                    TerrainType::Snow
                } else {
                    unreachable!()
                }
            })
            .collect();
        let terrain_map = TerrainMap::from_shape_vec(shape, terrain_map).unwrap();

        Self {
            terrain_map,
            height_map,
        }
    }
}
