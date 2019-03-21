use wasm_bindgen::prelude::*;

use vdom;

// developer facing api 

// commands to the runtime
pub enum Cmd<TMsg> {
    PortCommand (wasm_bindgen::JsValue),
    HttpCommand (TMsg, fn(Option<String>) -> TMsg),
    Composite(Box<Cmd<TMsg>>, Box<Cmd<TMsg>>)
}

pub enum Sub<TMsg> {
    TimeSub (TMsg)
}

pub fn build_component<Model, Message>(
    init: fn(&str) -> (Model, Option<Cmd<Message>>),
    update: fn(&Message, &Model) -> (Option<Model>, Option<Cmd<Message>>),
    view:  fn(&Model) -> vdom::Document,
    subscriptions: fn(&Model) -> Option<Sub<Message>>
) -> CobaltRuntime {
    unimplemented!();
}


// JsValue? or string?s
pub fn port_sub<Message>(name: &str, converter: fn(JsValue)-> Message) -> Sub<Message> {
    unimplemented!();
}

pub fn port_pub<Message>(name: &str, value: JsValue) -> Cmd<Message> {
     unimplemented!();
}

// todo: api to chain cmds together




// js facing api

#[wasm_bindgen]
pub struct CobaltRuntime {
    handlers: Vec<js_sys::Function> // todo: add endpointName
}

#[wasm_bindgen]
impl CobaltRuntime {
    pub fn post_msg(endpointName: &str, message: JsValue) {

    }

    pub fn sub_msg(&mut self, endpointName: &str, handler: js_sys::Function) { 
        // store function in array
        self.handlers.push(handler);
    }
}
