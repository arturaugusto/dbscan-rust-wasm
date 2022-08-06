<div align="center">

  <h1><code>dbscan-rust-wasm</code></h1>

  <strong>DBSCAN implemented in rust and compiled to WebAssembly.</strong>

</div>


## 🚴 Usage

```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build

to  use without bundler:

wasm-pack build --target web
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox

watch for changes:

cargo watch -ci .gitignore -i "pkg/*" -s "wasm-pack test --firefox --headless"
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.