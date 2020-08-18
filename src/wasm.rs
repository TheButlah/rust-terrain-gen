use super::*;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Starting wasm...".into());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("no document on window found");
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .expect("no canvas found")
        .dyn_into()?;
    let render_ctx: web_sys::CanvasRenderingContext2d =
        canvas.get_context("2d")?.unwrap().dyn_into()?;

    web_sys::console::log_1(&"Building Terrain...".into());
    let hmap = height_map::Builder {
        parallel: false,
        seed: thread_rng().gen(),
        tile_size: Some(200),
        ..Default::default()
    }
    .build(480, 640);
    let terrain = Terrain::from(hmap);

    web_sys::console::log_1(&"Displaying Terrain...".into());
    let cols = terrain.shape().1;
    let mut rgba = terrain.to_rgba();
    let img_data =
        web_sys::ImageData::new_with_u8_clamped_array(Clamped(&mut rgba), cols as u32)?;
    render_ctx.put_image_data(&img_data, 0.0, 0.0)?;

    Ok(())
}
