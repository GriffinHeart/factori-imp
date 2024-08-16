#[macro_use]
extern crate factori;

pub struct User {
  name: String,
}

factori!(User, {
  default {
    name: String = "Richard".to_string()
  }

  transient {
    upcased: bool = false
  }

  mixin upcased {
    upcased = true
  }

  builder {
    let name = if upcased { name.to_uppercase() } else { name };
    User { name }
  }
});


#[test]
fn transient_doesnt_change_anything() {
  let user = create!(User, name: "John".into());

  assert_eq!(user.name, "John");
}

#[test]
fn transient_changes_value() {
  let user = create!(User, upcased: true);

  assert_eq!(user.name, "RICHARD");
}

#[test]
fn transient_works_with_mixins() {
  let user = create!(User, :upcased);

  assert_eq!(user.name, "RICHARD");
}
