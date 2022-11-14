use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
}

pub fn fl_main(body: serde_json::Value) -> serde_json::Value {
    let parsed_body: Person = serde_json::from_value(body).expect("Failed to parse JSON in input");
    let out = format!("Hello {}!", parsed_body.name);
    serde_json::from_str(&format!(r#"{{"payload": "{}" }}"#, &out))
        .expect("Failed to parse JSON in project")
}
