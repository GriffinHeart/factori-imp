# factori-imp(roved)

A testing factory library for Rust, inspired by:

- [FactoryBot](https://github.com/thoughtbot/factory_bot). 🤖 🦀
- [mjkillough/factori](https://github.com/mjkillough/factori)

A fork of [mjkillough/factori](https://github.com/mjkillough/factori) library,
to add additional features.

factori-imp(roved) makes it easy to instantiate your test objects/fixtures in
tests while providing an ergonomic syntax for defining how they are
instantiated.

factori-imp works on stable Rust >=1.45.

## Differences with factori

- Transient attributes as first class citizens, see [tests/transient.rs](https://github.com/GriffinHeart/factori-imp/blob/main/tests/transient.rs)
- Adds `create_vec!` macro, see [tests/create_vec.rs](https://github.com/GriffinHeart/factori-imp/blob/main/tests/create_vec.rs)
- `create*` macros can be used in factory declarations, see [tests/simple:46](https://github.com/GriffinHeart/factori-imp/blob/main/tests/simple.rs#L46)
  - Contributed  by @wuerges, thank you
- Fixes all clippy warnings due to usage of the macros

## Documentation

[https://docs.rs/factori-imp/latest/factori_imp/](https://docs.rs/factori-imp/latest/factori_imp/)

### In place replacement for factori

You can use factori-imp without changing any code, in your Cargo.toml:

```diff
- factori = "1.1.0"
+ factori = { package = "factori-imp", version = "0.9.2" }
```

or just add it as a regular dependency and then rename the crate in your crate root:

`extern crate factori-imp as factori;`

## Example

factori-imp(roved) provides three macros:

- `factori!`, which defines a factory for a type
- `create!` which instantiates it
- `create_vec!` which instantiates many

```rust
#[macro_use]
extern crate factori;

pub struct Vehicle {
  number_wheels: u8,
  electric: bool,
}

factori!(Vehicle, {
  default {
    number_wheels = 4,
    electric = false,
  }

  mixin bike {
    number_wheels = 2,
  }
});

fn main() {
  let default = create!(Vehicle);
  assert_eq!(default.number_wheels, 4);
  assert_eq!(default.electric, false);

  // Its type is Vehicle, nothing fancy:
  let vehicle: Vehicle = default;

  let three_wheels = create!(Vehicle, number_wheels: 3);
  assert_eq!(three_wheels.number_wheels, 3);

  let electric_bike = create!(Vehicle, :bike, electric: true);
  assert_eq!(electric_bike.number_wheels, 2);
  assert_eq!(electric_bike.electric, true);

  // We can create many vehicles
  let many_motorcycles = create_vec!(Vehicle, 5, number_wheels: 2);
  assert_eq!(many_motorcycles.len(), 5);
}
```

More examples are available in the
[`tests/`](https://github.com/GriffinHeart/factori-imp/tree/main/tests) directory.

## Testing

Install [cargo-nextest](https://nexte.st/)

Run:

```sh
make test
```

## License

MIT
