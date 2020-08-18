use clap::Clap;
use rust_terrain_gen::*;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "2000")]
    rows: usize,

    #[clap(short, long, default_value = "2000")]
    cols: usize,

    #[clap(short, long, default_value = "1337")]
    seed: u32,

    /// Prevent the image from being outputted
    #[clap(long)]
    no_image: bool,

    /// Prevent running the noise generation in parallel
    #[clap(long)]
    no_parallel: bool,
}

fn main() {
    let opts = Opts::parse();
    let img_path = std::env::current_dir().unwrap();
    let img_path = img_path.join("my_terrain.png");

    let hmap = height_map::Builder {
        parallel: !opts.no_parallel,
        seed: opts.seed,
        tile_size: Some(200),
        ..Default::default()
    }
    .build(opts.rows, opts.cols);
    let terrain = Terrain::from(hmap);

    if !opts.no_image {
        terrain
            .to_img()
            .save(img_path)
            .expect("Was unable to save the image!");
    }
}
