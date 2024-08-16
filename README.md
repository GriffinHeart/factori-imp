# Factorio

A testing factory library for Rust, inspired by

[FactoryBot](https://github.com/thoughtbot/factory_bot). 🤖 🦀
[mjkillough/factori](https://github.com/mjkillough/factori)

A fork of [mjkillough/factori](https://github.com/mjkillough/factori) library,
to add additional features.

factorio makes it easy to instantiate your test objects/fixtures in tests while
providing an ergonomic syntax for defining how they are instantiated.

factorio works on stable Rust >=1.45.

## Documentation

See [API documentation](https://docs.rs/factorio/latest/factorio/).

## Example

factorio provides two macros: `factorio!`, which defines a factory for a type,
and `create!` which instantiates it:

```rust
#[macro_use]
extern crate factorio;

pub struct Vehicle {
    number_wheels: u8,
    electric: bool,
}

factorio!(Vehicle, {
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
}
```

More examples are available in the
[`tests/`](https://github.com/mjkillough/factorio/tree/master/tests) directory.

## Testing

Install [cargo-nextest](https://nexte.st/)

Run:

```sh
make test
```

## License

MIT
