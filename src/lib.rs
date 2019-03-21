extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate url;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use url::{Url};

mod vdom;
mod cobalt;


// ----- example

#[wasm_bindgen]
pub fn render() -> cobalt::CobaltRuntime {
    cobalt::build_component(init, update, view, |model| None)
}

fn init(data: &str) -> (Model, Option<cobalt::Cmd<Msg>>) {
    (Model {
        title: String::from("Test"),
        slogan: String::from("test slogan")
    }, None)
}

fn view(_: &Model) -> vdom::Document {
    vdom::Document{}
}

fn update(msg: &Msg, _: &Model) -> (Option<Model>, Option<cobalt::Cmd<Msg>>) {
    match msg {
        DataReceived => (Some(Model {
            title: String::from("Data"),
            slogan: String::from("received")
        }), None)
    }
}

struct Model {
    title: String,
    slogan: String
}

enum Msg {
    DataReceived
}



// define a set_panic_hook function, 
// if feature not activated, then it won't do anything
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn todoinit() {
    set_panic_hook();
}
