use std::fs;

use serde_json::Value;

pub fn load_json(file_path: &str) -> Value {
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    serde_json::from_str(&data).expect("Invalid JSON")
}

pub fn read_json(json: Value, offset: u32) -> Result<(), String> {
    if let Value::Object(map) = json {
        for (key, value) in map {
            for _ in 0..offset {
                print!("   ");
            }

            if let Value::Object(_) = value {
                println!("{}: ", key);
                read_json(value, offset + 1)?;
            } else {
                println!("{}: {}", key, value);
            }
        }
    } else {
        return Err("Not a JSON".to_string());
    }
    Ok(())
}
