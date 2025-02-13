
mod error;
mod value;
mod parameters;

pub use error::ParameterError;
pub use value::ParameterValue;
pub use parameters::{Parameters, ParametersBuilder};

pub type Result<T> = std::result::Result<T, ParameterError>;

#[macro_export]
macro_rules! parameters {
    () => {
        $crate::Parameters::new()
    };

    ($text:expr) => {
        $crate::Parameters::from($text)
    };

    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut params = $crate::Parameters::new();
        $(
            params = params.with($key, $value);
        )+
        params
    }};
}

#[cfg(test)]
mod tests {
    use crate::value::IntVec;

    use super::*;

    #[test]
    fn test_basic_usage() {
        let mut params = Parameters::new();
        params.insert("name", "Alice");
        params.insert("age", 30);
        params.insert("height", 5.9f64);
        params.insert("deceased", false);

        assert_eq!(params.get::<&str>("name"), Some("Alice").as_ref());
        assert_eq!(params.get::<i32>("age"), Some(&30));
        assert_eq!(params.get::<f64>("height"), Some(&5.9));
        assert_eq!(params.get::<bool>("deceased"), Some(&false));
    }

    #[test]
    fn test_builder_pattern() {
        let params = ParametersBuilder::new()
            .add("name", "Bob")
            .add("scores", IntVec::<i32>(vec![85, 92, 78]))
            .build();

        assert_eq!(params.get::<&str>("name"), Some("Bob").as_ref());
        assert_eq!(params.get::<IntVec<i32>>("scores").map(|v| v.0.clone()), Some(vec![85, 92, 78]));
    }

    #[test]
    fn test_macro() {
        let params = parameters! {
            "name" => "Charlie",
            "age" => "25",
        };

        assert_eq!(params.get_string("name"), Some("Charlie".to_string()));
        assert_eq!(params.get_string("age"), Some("25".to_string()));
    }

    #[test]
    fn test_custom_type() {
        #[derive(Debug, Clone)]
        struct Person {
            name: String,
            age: i32,
        }

        impl ToString for Person {
            fn to_string(&self) -> String {
                format!("{} ({})", self.name, self.age)
            }
        }

        let person = Person {
            name: "Dave".to_string(),
            age: 35,
        };

        let params = Parameters::new().with("person", person);
        let stored_person = params.get::<Person>("person").unwrap();

        assert_eq!(stored_person.name, "Dave");
        assert_eq!(stored_person.age, 35);
    }
}