#[macro_use]
extern crate factorio;

pub struct Vehicle {
  number_wheels: u8,
  electric: bool,
}

pub struct Passenger {
  name: &'static str,
}

pub struct Cargo {
  weight: u8,
}

// We can define multiple with one macro:
factorio!(
    Vehicle, {
        default {
            number_wheels = 4,
            electric = false,
        }
    }

    Passenger, {
        default {
            name = "Michael"
        }
    }
);

// Or call the macro twice:
factorio!(
    Cargo, {
        default {
            weight = 0
        }
    }
);

#[test]
fn vehicle() {
  let default = create!(Vehicle);
  assert_eq!(default.number_wheels, 4);
  assert_eq!(default.electric, false);
}

#[test]
fn passenger() {
  let default = create!(Passenger);
  assert_eq!(default.name, "Michael");
}

#[test]
fn cargo() {
  let default = create!(Cargo);
  assert_eq!(default.weight, 0);
}

#[test]
fn override_field() {
  let tom = create!(Passenger, name: "Tom");
  assert_eq!(tom.name, "Tom");
}

#[test]
fn multiple_fields() {
  let multiple = create!(Vehicle, number_wheels: 8, electric: true);
  assert_eq!(multiple.number_wheels, 8);
  assert_eq!(multiple.electric, true);
}
