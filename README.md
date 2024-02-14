# README
```
 rustup override set nightly
 rustup target add wasm32-unknown-unknown
 trunk serve --open
 ```
 The table column's header ("Name") should sort the column alphabetically, instead it causes a panic (unreachable code) visible in the browser's console.

 Interestingly, removing the `#[table(skip)]` decorator on Item's `id` field will prevent this bug from occurring.