#[macro_use]
extern crate factori_imp;

pub struct Vehicle {
  number_wheels: u8,
  electric: bool,
}

factori!(Vehicle, {
  default {
    number_wheels: u8 = 4,
    electric: bool = false,
  }

  transient {
    double_wheels: bool = false
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

  builder {
    let number_wheels = if double_wheels {
      number_wheels * 2
    } else { number_wheels };

    Vehicle { number_wheels, electric }
  }
});

#[test]
fn can_create_many() {
  let vehicles = create_vec!(Vehicle, 5);

  assert_eq!(vehicles.len(), 5);
}

#[test]
fn works_with_mixins() {
  let vehicles = create_vec!(Vehicle, 3, :bike);

  vehicles.iter().for_each(|vehicle| {
    assert_eq!(vehicle.number_wheels, 2);
  });
}

#[test]
fn works_with_overriding_fields() {
  let vehicles = create_vec!(Vehicle, 3, electric: true);

  vehicles.iter().for_each(|vehicle| {
    assert!(vehicle.electric);
  });
}

#[test]
fn works_with_transient_fields() {
  let vehicles = create_vec!(Vehicle, 3, double_wheels: true);

  vehicles.iter().for_each(|vehicle| {
    assert_eq!(vehicle.number_wheels, 8);
  });
}

#[test]
fn works_with_all_of_them() {
  let vehicles = create_vec!(Vehicle, 3, :bike, electric: true, double_wheels: true);

  vehicles.iter().for_each(|vehicle| {
    assert_eq!(vehicle.number_wheels, 4);
    assert!(vehicle.electric);
  });
}
