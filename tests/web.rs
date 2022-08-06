//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate dbscan_rust_wasm;
use dbscan_rust_wasm::*;


wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {

    let js_params = JsParams {
        eps: 1.0,
        min_points: 3,
        inputs: vec![
            vec![1.5, 2.2],
            vec![1.0, 1.1],
            vec![1.2, 1.4],
            vec![0.8, 1.0],
            vec![3.7, 4.0],
            vec![3.9, 3.9],
            vec![3.6, 4.1],
            vec![10.0, 10.0],
        ],
    };

    let output = run_model(&JsValue::from_serde(&js_params).unwrap());

    assert_eq!(
        output,
        vec![
            -1.,
            0.,
            0.,
            0.,
            1.,
            1.,
            1.,
            -1.
        ]
    );

}
