# rust-site

- `cargo new --lib rust-site`
- `mkdir static`
- `wasm-pack build --target web --out-name wasm --out-dir ./static`
- `python3 -m http.server 8000`
- `npx gh-pages -b gh-pages -d .`
- `cargo install cargo-web`
  - needs std-web
- `npm install -g local-web-server`
- `ws --spa index.html`
- `wasm-pack build --target web --out-name wasm --out-dir ./static && cd static && ws --spa index.html`

## wishlist

- better jsx editor support
  - autocomplete
  - errors just highlight the whole macro
  - no jumping to definition
- jsx without `{}` everywhere
- hot reloading
- better deployment
- code splitting?
- server side rendering
- react style passing state handler to child?

## todo

- webpack
- <mark>