mod utils;


mod conways_game_of_life;
mod universe_2d_screen;
mod colors;

use std::cell::RefCell;
use std::rc::Rc;
use std::f64;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::conways_game_of_life::{Universe, Cell};
use crate::universe_2d_screen::{Universe2DScreen};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    match window().request_animation_frame(f.as_ref().unchecked_ref()) {
        _ => {}
    }
}


fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen]
struct GameOfLife {
    request_id: Option<i32>,
    canvas_id: String,
    animation_frame_request: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

#[wasm_bindgen]
impl GameOfLife {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> GameOfLife {
        GameOfLife {
            request_id: None,
            canvas_id: canvas_id.to_string(),
            animation_frame_request: Rc::new(RefCell::new(None)),
        }
    }

    fn get_canvas_2d_context(&self) -> web_sys::CanvasRenderingContext2d {
        let document = document();
        let canvas = document.get_element_by_id(&self.canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width(385);
        canvas.set_height(385);

        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }


    #[wasm_bindgen]
    pub fn start_game_of_life(&mut self) {
        let context = self.get_canvas_2d_context();
        let universe = Universe::new();

        let mut screen = Universe2DScreen::new(context, universe);

        let g: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = self.animation_frame_request.clone();

        screen.draw();

        let draw: Box<dyn FnMut()> = Box::new(move || {
            screen.draw();
            request_animation_frame(g.borrow_mut().as_ref().unwrap());
        });

        let draw = Closure::wrap(draw);

        *self.animation_frame_request.borrow_mut() = Some(draw);

        request_animation_frame(self.animation_frame_request.borrow_mut().as_ref().unwrap());
    }

    #[wasm_bindgen]
    pub fn stop_game_of_life(&mut self) -> Result<(), JsValue> {
        self.animation_frame_request.borrow_mut().take();
        Ok(())
    }
}