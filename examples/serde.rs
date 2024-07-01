use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };

    let json = serde_json::to_string(&user)?;
    println!("json: {}", json);

    let user: User = serde_json::from_str(&json)?;
    println!("user: {:?}", user);

    Ok(())
}
