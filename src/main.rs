use rust_terrain_gen::*;

fn main() {
    let seed = 1337;
    let rows = 150;
    let cols = 100;
    let img_path = std::env::current_dir().unwrap();
    let img_path = img_path.join("my_terrain.png");

    let hmap = height_map::Builder::default().seed(seed).build(rows, cols);
    let terrain = Terrain::from(hmap);

    terrain
        .to_img()
        .save(img_path)
        .expect("Was unable to save the image!");
}
