// will eventually get better, as i move up chapters in the Book

use serde_json;
use std::collections::HashMap;

struct Row {
    id: u32,
    name: String,
    age: u8,
}

fn row_to_map(row: &Row) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("id".to_string(), row.id.to_string());
    map.insert("name".to_string(), row.name.clone());
    map.insert("age".to_string(), row.age.to_string());

    map
}

fn map_to_json(map: &HashMap<String, String>) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string(&map)?;
    Ok(json)
}

#[allow(unused)]
fn json_to_map(json_str: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let map = serde_json::from_str(&json_str)?;
    Ok(map)
}

fn main() {
    let row = Row {
        id: 1,
        name: String::from("Great"),
        age: 18,
    };

    let map = row_to_map(&row);

    let json = map_to_json(&map).unwrap();
    println!("{}", json);

    let back: HashMap<String, String> = json_to_map(&json).unwrap();
    println!("{:?}", back);
}
