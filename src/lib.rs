mod utils;

use wasm_bindgen::prelude::*;
use dbscan::*;
use serde::{Serialize, Deserialize};
use Classification::{Core, Edge, Noise};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[derive(Serialize, Deserialize)]
pub struct JsParams {
    pub eps: f64,
    pub min_points: usize,
    pub inputs: Vec<Vec<f64>>,
}

#[wasm_bindgen]
pub fn run_model(js_object: &JsValue) -> Vec<f64> {

    let js_params: JsParams = js_object.into_serde().unwrap();

    let model = Model::new(js_params.eps, js_params.min_points);
    
    let inputs = js_params.inputs;

    let output = model.run(&inputs);

    let mut js_output = Vec::new();
    
    for item in output {
        match item {
            Core(c) => js_output.push(c as f64),
            Edge(_c) => js_output.push(-1 as f64),
            Noise => js_output.push(-1 as f64),
        };
    }
    js_output
}

#[wasm_bindgen]
pub fn greet() {

    alert("Hello, dbscan-rust-wasm!");
}
