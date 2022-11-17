
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

## Learning Material
* [too many list](https://rust-unofficial.github.io/too-many-lists/)
* 

## Documentation
generate documentation in current project
`cargo doc --open`

## General concept:
### Variable and Mutability
* variables:
  * by default, it is immutable
* constant 
  * always immutable 
  * constant expression 
  * need with type 
  * all uppercase with underscores between words.
    * `THREE_HOURS_IN_SECONDS`
  * Constant are valid for the entire time a program runs.
* Shadowing:
  * reuse the variable name and can change the type. if use `mut`, cannot change the type

### Data Types
* scalar types:
  * integers
  * float-point
  * Booleans
  * characters
    * single quote for char
* Compound types:
  * tuples:
    * fixed length, can assign type one by one
    * using a period(.) followed by index of vaLUE WE want to access
  * Array:
    * use `{:?}`: to print each element in the array
    * can declear [type; length] or [value; length]
    
### function
* statement and expression
  * statement: instructions that perform some action and o not return a value
    * `let x = 6`
    * Wrong: `let x = (let y = 6)`
    * Correct in C++: `x = y = 6`
  * expression: evaluate to a resulting value
  * `let y = 6`:
    * `6` is expression
    * `let y = 6` is a statement
  * a new scope block created with curly brackets is an expression
    * ```
      let y = {
          let x = 3;
          x + 1
      };
      ```
      
### Control Flow
* loop label:
  * to break different loop, we can use loop label

## Ownership:
* most unique feature, enable rust to make memory safety guarantees without garbage collector











  
  
  
  