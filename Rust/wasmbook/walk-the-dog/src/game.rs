use std::collections::HashMap;

use crate::{engine::{Game,Renderer, self, Rect}, browser};
use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
use web_sys::HtmlImageElement;

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell  {
    frame: SheetRect,
}

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    sheet: Option<Sheet>,
    image: Option<HtmlImageElement>,
    frame: u8,
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self {
            sheet: None,
            image: None,
            frame: 0,
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let sheet: Sheet = browser::fetch_json("rhb.json").await?.into_serde()?;

        let image = engine::load_image("rhb.png").await?;

        Ok(Box::new(WalkTheDog {
            sheet: Some(sheet),
            image: Some(image),
            frame: self.frame,
        }))
    }

    fn update(&mut self) {
        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let current_sprite = (self.frame / 3 ) + 1;
        let frame_name = format!("Run ({}).png", current_sprite);
        let sprite = self.sheet.as_ref().and_then(|sheet| sheet.frames.get(&frame_name)).expect("no frame found");

        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        self.image.as_ref().map(|image| {
        renderer.draw_iamge(
            &image,
            &Rect {
                x: sprite.frame.x.into(),
                y: sprite.frame.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
            &Rect {
                x: 300.0,
                y: 300.0,
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            }
        );
    });
    }
}