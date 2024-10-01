#[macro_use]
extern crate factori_imp;

pub struct Vehicle {
  number_wheels: u8,
  electric: bool,
}

pub struct Garage {
  vehicle: Vec<Vehicle>,
}

factori!(Vehicle, {
  default {
    number_wheels = 4,
    electric = false,
  }

  mixin bike {
    number_wheels = 2,
  }

  mixin trike {
    number_wheels = 3,
  }

  mixin electric {
    electric = true,
  }
});

factori!(Garage, {
  default {
    vehicle = create_vec!(Vehicle, 3),
  }
});

#[test]
fn simple_struct() {
  let default = create!(Vehicle);
  assert_eq!(default.number_wheels, 4);
  assert!(!default.electric);
}

#[test]
fn nested_struct() {
  let default = create!(Garage);
  assert_eq!(default.vehicle.len(), 3);
  assert_eq!(default.vehicle[0].number_wheels, 4);
  assert_eq!(default.vehicle[0].electric, false);
}

#[test]
fn override_field() {
  let three_wheels = create!(Vehicle, number_wheels: 3);
  assert_eq!(three_wheels.number_wheels, 3);
}

#[test]
fn one_mixin() {
  let bike = create!(Vehicle, :bike);
  assert_eq!(bike.number_wheels, 2);
  assert!(!bike.electric);
}

#[test]
fn mixin_and_override() {
  let electric_bike = create!(Vehicle, :bike, electric: true);
  assert_eq!(electric_bike.number_wheels, 2);
  assert!(electric_bike.electric);
}

#[test]
fn two_mixins() {
  let electric_bike = create!(Vehicle, :bike, :electric);
  assert_eq!(electric_bike.number_wheels, 2);
  assert!(electric_bike.electric);
}

#[test]
fn mixin_precedence() {
  let electric_bike = create!(Vehicle, :bike, :trike);
  assert_eq!(electric_bike.number_wheels, 3);
}
