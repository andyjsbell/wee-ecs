# wee-ecs

A lightweight, embedded-friendly Entity Component System (ECS) library for Rust.

## Features

- **Minimal footprint** - Designed for resource-constrained environments
- **no_std compatible** - Works without the standard library
- **Bitset-based architecture** - Efficient component lookup using bitmasks
- **Configurable world sizes** - Choose entity/component counts based on your needs
- **Type-erased components** - Store any Rust type as a component

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
wee-ecs = "0.1.0"
```

For `no_std` environments:

```toml
[dependencies]
wee-ecs = { version = "0.1.0", default-features = false, features = ["nostd"] }
```

## Usage

```rust
use wee_ecs::prelude::*;
use std::cell::RefCell;

// Define components
struct Name(String);
struct Age(u32);

fn main() {
    // Create a world
    let world = PrimitiveWorld::new("my world");

    // Register component types
    PrimitiveWorld::register::<Name>();
    PrimitiveWorld::register::<Age>();

    // Spawn entities with components
    let world = world
        .spawn(vec![
            RefCell::new(Box::new(Name("Alice".to_owned()))),
            RefCell::new(Box::new(Age(30))),
        ])
        .spawn(vec![
            RefCell::new(Box::new(Name("Bob".to_owned()))),
        ]);

    // Query entities by component
    let mask = Query::<Name>::query();
    let entities = world.query(mask);

    // Add and run systems
    world.add_system(mask, |_mask, _entities| {
        // Process matching entities
    });
    world.run();
}
```

## World Types

| Type | Max Entities | Max Components |
|------|--------------|----------------|
| `PrimitiveWorld` | 8 | 8 |
| `World` | 128 | 128 |

You can also create custom world configurations using `GenericWorld<T, U>`.

## License

See [Cargo.toml](Cargo.toml) for license information.
