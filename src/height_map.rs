#[cfg(not(target_arch = "wasm32"))]
use indicatif::ProgressIterator;

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
    pub center_height: Height,
    pub scale: Height,
    pub parallel: bool,
    /// How many pixels for one edge of the first octave's region. If `None`,
    /// will use the larger of the width or the height specified when building
    pub tile_size: Option<usize>,
    pub seed: u32,
}
impl Default for Builder {
    fn default() -> Self {
        Self {
            octaves: 6,
            persistence: 0.4,
            center_height: 0.0,
            scale: 1.0,
            tile_size: None,
            parallel: true,
            seed: Perlin::DEFAULT_SEED,
        }
    }
}
impl Builder {
    pub fn build(&self, rows: usize, cols: usize) -> HeightMap {
        // Handle options
        assert!(self.octaves > 0, "There must be at least 1 octave");
        assert_ne!(self.tile_size, Some(0), "Tile size cannot be zero!");
        let tile_size =
            self.tile_size.unwrap_or_else(|| std::cmp::max(rows, cols)) as f64;
        let hmap_shape = (rows, cols);

        let perlin = noise::Perlin::new().set_seed(self.seed);

        let amplitudes: Vec<_> = (0..self.octaves)
            .map(|i| self.persistence.powi(i as i32))
            .collect();
        let max_val = amplitudes.iter().sum::<f64>() as Height;

        let mut hmap = HeightMap::zeros(hmap_shape);

        let gen_fn = |(r, mut the_row)| {
            // println!("Computing row {}", r);
            for (octave, amplitude) in amplitudes.iter().enumerate() {
                let input_scale = 2f64.powi(octave as i32) / tile_size;
                let octave_height_fn = |c| {
                    let (y, x) = (r as f64, c as f64);
                    let x = (x + tile_size) * input_scale;
                    let y = (y + tile_size) * input_scale;

                    perlin.get([y, x]) as Height
                };
                let mut tmp_row =
                    ndarray::Array1::from_shape_fn(hmap_shape.1, octave_height_fn);

                tmp_row *= *amplitude as Height;
                the_row += &tmp_row;
            }
        };

        // the progress bar even knows how many elements there will be!!
        let iter = hmap.outer_iter_mut().enumerate();

        #[cfg(not(target_arch = "wasm32"))]
        let iter = iter.progress();

        if self.parallel {
            iter.par_bridge().for_each(gen_fn);
        } else {
            iter.for_each(gen_fn);
        };

        hmap *= (self.scale / max_val) as Height;
        hmap += self.center_height;
        hmap
    }
}

#[cfg(test)]
mod tests {}
