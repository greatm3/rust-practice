// will eventually get better, as i move up chapters in the Book

use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    id: u32,
    name: String,
    age: u8,
}

fn map_to_json(row: &Row) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string(&row)?;
    Ok(json)
}

fn json_to_map(json_str: &str) -> Result<Row, serde_json::Error> {
    let map = serde_json::from_str(&json_str)?;
    Ok(map)
}

fn main() {
    let row = Row {
        id: 1,
        name: String::from("Great"),
        age: 18,
    };

    let json = map_to_json(&row).unwrap();
    println!("{}", json);

    let back = json_to_map(&json).unwrap();
    println!("{:?}", back);
}
