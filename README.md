# wasm_demo

Here's how to run the demo:

1. Install Rust (https://www.rust-lang.org/). The project is written in Rust.
2. Install native Perl (https://strawberryperl.com/ for Windows. For MacOS and Linux, use your preferred package manager.). A dependency of this project uses Perl scripting in its compilation.
3. Install Python. This is used to start a simple web server, to circumvent CORS restrictions. You may instead use a different web server.
4. Clone this repo
5. Run `rustup target add wasm32-unknown-unknown` to install the WebAssembly target to the Rust compiler
6. Run `cargo install wasm-pack` to install `wasm-pack`, a CLI tool for building WebAssembly packages
7. Run `cargo update -p tracing-wasm --precise 0.2.0` to circumvent a current bug in the dependencies, caused by a semantic versioning breakage
8. Run `wasm-pack build --target web --release` to build the project. Allow this command to completely finish.
9. Move `wasm_demo_bg.wasm` and `wasm_demo.js` from pkg/ to the repo's root
10. Run `python -m SimpleHTTPServer` or `python3 -m http.server` to start the web server
11. Open `http://localhost:8000/` in your browser

To rebuild the demo, follow the steps from 8 to 11.

## Troubleshooting

If you run into any issues, all of these technologies are in early stages, so finding support online will be difficult.

If you attempt to build the demo natively, delete `target/` before doing so, and delete it again before rebuilding for web.

Some IDEs will update JavaScript imports automatically when you move the files. Make sure that this does not happen.