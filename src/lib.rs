pub mod height_map;
mod terrain;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use terrain::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
