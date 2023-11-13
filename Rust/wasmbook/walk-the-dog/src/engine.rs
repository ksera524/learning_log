use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::browser::{self, LoopClosure};

const FRAME_SIZE:f32 = 1.0/60.0 * 1000.0;// 60 fps

pub trait Game {
    fn update(&mut self);
    fn draw(&self, context: &CanvasRenderingContext2d);
}

pub struct GameLoop {
    last_frame: f64,
    accumulated_delta:f32,
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(mut game: impl Game + 'static) -> Result<()> {
        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(browser::create_ref_clousure(
            move |perf: f64| {
                game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
                while game_loop.accumulated_delta >= FRAME_SIZE {
                    game.update();
                    game_loop.accumulated_delta -= FRAME_SIZE;
                }
                game_loop.last_frame = perf;
                game.draw(&browser::context().expect("Error getting context"));

                browser::request_animation_frame(f.borrow().as_ref().unwrap());
            },
        ));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("no canvas found"))?,
        )?;
        Ok(())
    }
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;

    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(()));
        }
    });

    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx.send(Err(anyhow!("Error loading image: {:?}", err)));
        }
    });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

    image.set_src(source);

    complete_rx.await??;

    Ok(image)
}
