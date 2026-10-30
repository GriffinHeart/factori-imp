//! Regression test: `create!` and `create_vec!` must expand to a single
//! expression, without a trailing semicolon.
//! See https://github.com/rust-lang/rust/issues/79813
#![deny(semicolon_in_expressions_from_non_local_macros)]

#[macro_use]
extern crate factori_imp;

pub struct Vehicle {
  number_wheels: u8,
}

factori!(Vehicle, {
  default {
    number_wheels = 4,
  }
});

fn wheels(vehicle: Vehicle) -> u8 {
  vehicle.number_wheels
}

#[test]
fn create_in_let_binding() {
  let vehicle = create!(Vehicle);
  assert_eq!(vehicle.number_wheels, 4);
}

#[test]
#[allow(clippy::useless_vec)]
fn create_in_vec_macro() {
  let vehicles = vec![create!(Vehicle), create!(Vehicle, number_wheels: 2)];
  assert_eq!(vehicles.len(), 2);
}

#[test]
fn create_as_function_argument() {
  assert_eq!(wheels(create!(Vehicle, number_wheels: 3)), 3);
}

#[test]
fn create_vec_in_let_binding() {
  let vehicles = create_vec!(Vehicle, 2);
  assert_eq!(vehicles.len(), 2);
}

#[test]
fn create_vec_as_function_argument() {
  assert_eq!(Vec::len(&create_vec!(Vehicle, 3)), 3);
}
