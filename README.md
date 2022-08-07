<div align="center">

  <h1><code>dbscan-rust-wasm</code></h1>

  <strong>DBSCAN implemented in rust and compiled to WebAssembly.</strong>
  <div>
    Based on https://github.com/lazear/dbscan
  </div>

</div>


## 🚴 Usage



### 🛠️ Build with `wasm-pack build`

```
wasm-pack build


or to use it without bundler:

wasm-pack build --target web
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox

to watch for changes:

cargo watch -ci .gitignore -i "pkg/*" -s "wasm-pack test --firefox --headless"
```

## Sample usage on a HTML page

```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script type="module">
      import init, { run_model } from './pkg/dbscan_rust_wasm.js';

      async function run() {
        await init();

        const result = run_model({
          eps: 1.0,
          min_points: 3,
          inputs: [
            [1.5, 2.2],
            [1.0, 1.1],
            [1.2, 1.4],
            [0.8, 1.0],
            [3.7, 4.0],
            [3.9, 3.9],
            [3.6, 4.1],
            [10.0, 10.0],
          ],
        });

        console.log(result);
      }

      run();
    </script>
  </body>
</html>
```

Note: Page must be served from a web-server (e.g: python -m http.server)

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