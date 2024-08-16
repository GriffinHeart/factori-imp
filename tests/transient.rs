#[macro_use]
extern crate factori;

pub struct User {
  name: String,
}

factori!(User, {
  default {
    name = "Richard".to_string(),
  }

  transient {
    upcased: bool = false,
  }
});

#[test]
fn transient_doesnt_change_anything() {
  let user = create!(User, name: "John".into());

  assert_eq!(user.name, "John");
}
