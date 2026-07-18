use serde_json;
use std::collections::HashMap;

struct Row {
    id: u32,
    name: String,
    age: u8,
}

fn row_to_map(row: &Row) -> HashMap<String, String> {
    todo!()
}

fn map_to_json(map: &HashMap<String, String>) -> String {
    todo!()
}

fn main() {
    let row = Row {
        id: 1,
        name: "Great".to_string(),
        age: 18,
    };

    let map = row_to_map(&row);
    let json = map_to_json(&map);

    println!("{}", json);
}
