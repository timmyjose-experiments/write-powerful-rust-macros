use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Request {
    given_name: String,
    last_name: String,
}

fn full_name(given_name: &str, last_name: &str) -> String {
    format!("{given_name} {last_name}")
}

fn main() {
    let req = Request {
        given_name: "Bob".to_string(),
        last_name: "Danvers".to_string(),
    };

    dbg!(full_name(&req.given_name, &req.last_name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let person: Request = serde_json::from_str(
            "{
        \"given_name\": \"Bob\",
        \"last_name\": \"Danvers\"
        }",
        )
        .expect("deserialize to work");

        assert_eq!("Bob", person.given_name);
        assert_eq!("Danvers", person.last_name);
    }
}
