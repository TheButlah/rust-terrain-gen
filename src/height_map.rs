use ndarray::parallel::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rayon::iter::ParallelBridge;

pub type Height = f32;
pub type HeightMap = ndarray::Array2<Height>;

/// For each octave `i`, the frequency is `2^i` and the amplitude is `p^i`,
/// where `p` is the persistence. Typically, the persistence is below 1 and
/// above 0.
pub struct Builder {
    pub octaves: u8,
    pub persistence: f64,
    pub center_height: f64,
    pub scale: f64,
    /// How many pixels for one edge of the first octave's region. If `None`,
    /// will use the larger of the width or the height specified when building
    pub tile_size: Option<usize>,
    seed: u32,
    noise: Perlin,
}
impl Default for Builder {
    fn default() -> Self {
        Self {
            octaves: 8,
            persistence: 0.4,
            center_height: 0.0,
            scale: 1.0,
            tile_size: None,
            seed: Perlin::DEFAULT_SEED,
            noise: Perlin::new(),
        }
    }
}
impl Builder {
    pub fn seed(&mut self, seed: u32) -> &mut Self {
        self.seed = seed;
        self.noise.set_seed(seed);
        self
    }

    pub fn build(&self, rows: usize, cols: usize) -> HeightMap {
        assert!(self.octaves > 0, "There must be at least 1 octave");
        assert_ne!(self.tile_size, Some(0), "Tile size cannot be zero!");
        let tile_size = self.tile_size.unwrap_or(std::cmp::max(rows, cols)) as f64;

        let mut map = HeightMap::from_elem((rows, cols), 0 as Height);
        for i in 0..self.octaves {
            println!("Octave {}", i);
            let amplitude = self.persistence.powi(i as i32);
            let horizontal_scale = 2f64.powi(i as i32) / tile_size;
            map.indexed_iter_mut().for_each(|((row, col), height)| {
                let i = i as i32;
                let (y, x) = (row as f64, col as f64);
                let y = y * horizontal_scale;
                let x = x * horizontal_scale;
                // println!("x,y: {},{}", x, y);

                let octave_height = self.noise.get([y, x]);
                // println!("Height: {}", octave_height);
                let octave_height = (octave_height * amplitude) as Height;
                // println!("Height: {}", octave_height);

                *height += octave_height;
                // println!("Height: {}", height);
            });
        }

        map *= self.scale as Height;
        map += self.center_height as Height;
        map
    }
}
