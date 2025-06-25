use std::fs;

use serde_json::Value;

fn add_offset(offset: u32) {
    for _ in 0..offset {
        print!("   ")
    }
}

pub fn load_json(file_path: &str) -> Value {
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    serde_json::from_str(&data).expect("Invalid JSON")
}

fn show_array(array: Vec<Value>, offset: u32) -> Result<(), String> {
    for val in array {
        match val {
            Value::Array(arr) => {
                add_offset(offset);
                show_array(arr, offset)?
            }
            Value::Object(_) => show_json(val, offset)?,
            _ => {
                add_offset(offset);
                println!("{}", val)
            }
        }
    }
    Ok(())
}

pub fn show_json(json: Value, offset: u32) -> Result<(), String> {
    if let Value::Object(map) = json {
        for (key, value) in map {
            add_offset(offset);
            print!("{}:", key);
            match value {
                Value::Array(arr) => {
                    println!();
                    show_array(arr, offset + 1)?
                }
                Value::Object(_) => {
                    println!();
                    show_json(value, offset + 1)?
                }
                _ => println!(" {}", value),
            }
        }
    } else {
        return Err("Not a JSON".to_string());
    }
    Ok(())
}
