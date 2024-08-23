# Backend of pyum

- Run api server with watch mode
```sh
cargo run
```

- One shot run api server
```sh
cargo run
```

- Run test
  - In executing with multi thread, model deletion is not works expected, so thread should be only one.
run below test: cargo test -- --nocapture --test-threads=1

```sh
cargo test -- --nocapture --test-threads=1
```
