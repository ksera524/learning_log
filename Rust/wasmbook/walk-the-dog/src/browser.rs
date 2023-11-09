use anyhow::{anyhow, Result};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Response, Window};

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("no window found"))
}

pub fn document() -> Result<web_sys::Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("no document found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("no canvas found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error casting to HtmlCanvasElement: {:?}", element))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting context: {:?}", js_value))?
        .ok_or_else(|| anyhow!("no context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|context| anyhow!("Error casting to CanvasRenderingContext2d: {:?}", context))
}

pub fn spawn_local<F>(future: F)
where
    F: futures::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
    JsFuture::from(window()?.fetch_with_str(resource))
        .await
        .map_err(|err| anyhow!("err casting to JsFuture: {:?}", err))
}

pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
    let resp_value = fetch_with_str(json_path).await?;
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|err| anyhow!("err casting to Response: {:?}", err))?;
    JsFuture::from(
        resp.json()
            .map_err(|err| anyhow!("err casting to JsFuture: {:?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("err casting to JsFuture: {:?}", err))
}
