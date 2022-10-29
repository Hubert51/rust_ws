
## Rust structure:
* if only define `[package]`, it will find `src/main.rs`
* if define `[package]` and `[bin]`, and the `name` in `[bin]` is `rust_ws`, then the project 
  will run the file with `path`. The following example, we do not need `main.rs`:
  ```
    [package]
    name = "rust_ws"
    version = "0.1.0"
    edition = "2021"
    
    [[bin]]
    name = "rust_ws"
    path = "src/main3.rs"
  ```
  
https://doc.rust-lang.org/cargo/guide/project-layout.html
```
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

## documentation
generate documentation in current project
`cargo doc --open`