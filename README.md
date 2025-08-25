# el-macro

a dumb macro collection for anti-idiomatic rust programming


## Basic usage

More comprehensive usage examples can be found on [docs.rs](https://docs.rs/el-macro/latest/el_macro/index.html).

### `bind!`

Binds to the unwrapped value or evaluates the execution flow control expression.
An optional error handler can be used, and support for custom types can be implemented. 

```rust
bind!(x = Some(42), or return);
assert_eq!(x, 42);

bind!(mut x = Some(42), or return);
x += 3;
assert_eq!(x, 45);

let handle_error = |err: &str| eprintln!("{err}!");
// prints 'error!' and returns
bind!(x = None::<i32>.ok_or("error"), or handle_error, return);
unreachable!();
```


### `if_matches!`

Maps pattern-bound variables to `Some` if the provided expression matches the pattern.

```rust
let a = Some(41);
let b = Some(43);
let avg = |x: i32, y: i32| (x + y) / 2;

let x = if_matches!((a, b), (Some(x), Some(y)) => avg(x, y));
assert!(x.is_some_and(|val| val == 42));

let x = if_matches!((a, None::<u8>), (Some(x), Some(_)) => a);
assert!(x.is_none());
```

Syntax similar to match guard is supported:

```rust
let vol = Some(100);
let bins = Some(0);
let per_bin = if_matches!((vol, bins), (Some(v), Some(b)) if b != 0 => v / b);
assert!(per_bin.is_none());
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
