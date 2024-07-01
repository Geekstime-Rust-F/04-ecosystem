use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Default, Builder, Debug)]
#[builder(build_fn(name = "_private_build"))]
struct User {
    name: String,

    #[builder(setter(into, strip_option), default)]
    email: Option<String>,

    #[builder(setter(custom))]
    date_of_birth: DateTime<Utc>,

    #[builder(setter(skip))]
    age: i32,

    #[builder(setter(into))]
    address: String,
}

impl User {
    fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    fn date_of_birth(&mut self, value: impl Into<String>) -> &mut Self {
        self.date_of_birth = Some(
            DateTime::parse_from_rfc3339(&value.into())
                .unwrap()
                .with_timezone(&Utc),
        );
        self
    }

    fn build(&mut self) -> Result<User, Box<dyn std::error::Error>> {
        let mut user = self._private_build()?;
        user.age = Utc::now().year() - user.date_of_birth.year();
        Ok(user)
    }
}

fn main() {
    let user = User::build()
        .name("F".to_string())
        .email("test@test.com")
        .date_of_birth("2021-01-01T00:00:00Z")
        .address("Shanghai")
        .build()
        .unwrap();
    println!("{:?}", user);
}
