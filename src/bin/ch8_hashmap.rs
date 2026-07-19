use serde_json;

fn main() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2468902"
            ]
        }
        "#;

    let v: serde_json::Value = serde_json::from_str(data).expect("REASON");

    println!("v: {v}")
}
