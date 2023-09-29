use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;
use rand::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image = web_sys::HtmlImageElement::new().unwrap();
    image.set_src("./static/Idle (1).png");

    sierpinski(&context, [(300.0,0.0),(0.0,600.0),(600.0,600.0)],(0,255,0), 5);

    Ok(())
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points:[(f64,f64);3],color:(u8,u8,u8)) {
    let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
    context.fill();
}

fn sierpinski(context:&web_sys::CanvasRenderingContext2d, points:[(f64,f64);3],color:(u8,u8,u8), depth:u32) {
    draw_triangle(&context, points, color);

    let depth = depth - 1;  

    if depth > 0 {
        let mut rng = rand::thread_rng();

        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255)
        );

        let [top, left, right] = points;
        let left_mid = midpoint(top, left);
        let right_mid = midpoint(top, right);
        let bottom_mid = midpoint(left, right);

        sierpinski(&context, [top, left_mid, right_mid], next_color,depth);
        sierpinski(&context, [left_mid, left, bottom_mid],next_color,depth);
        sierpinski(&context, [right_mid, bottom_mid, right] ,next_color,depth);
    }
}

fn midpoint(p1:(f64,f64), p2:(f64,f64)) -> (f64,f64) {
    ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0)
}
