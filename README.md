# Arena-rs

[![Crates.io](http://img.shields.io/crates/v/arena-rs.svg?label=arena-rs)](https://crates.io/crates/arena-rs)

An arena memory pool for fast Rust heap allocations and dealocations.

This allows data to be allocated instantly and destroyed in batches.

## Creating an Arena

```rust
let mut arena = Arena::new(1024 /* size in bytes*/);
```

## Allocating data

```rust
let value = arena.alloc(17)?;
assert_eq!(*value, 17);
```

## Deallocating data

```rust
{
    let arena = Arena::new(1024);
} // <- All data in arena is deallocated here
```

## Saftey

```rust
let arena = Arena::new(1024);
let value = arena.alloc(17)?;
move_arena(arena);
println!("{}", *value); // <- Compiler Error
// The arena was destroyed when we gave it to `move_arena`
// so we cannot use any data it gave to us
```
