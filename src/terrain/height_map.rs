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
        let mut map = HeightMap::from_elem((rows, cols), 0 as Height);
        for i in 0..self.octaves {
            map.indexed_iter_mut()
                .par_bridge()
                .for_each(|((row, col), height)| {
                    let i = i as i32;
                    let (y, x) = (row as f64, col as f64);
                    let y = y * 2f64.powi(i);
                    let x = x * 2f64.powi(i);
                    let amplitude = self.persistence.powi(i);

                    *height = *height + (self.noise.get([y, x]) * amplitude) as Height;
                });
        }

        map *= self.scale as Height;
        map += self.center_height as Height;
        map
    }
}
