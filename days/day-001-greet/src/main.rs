//! Day 001 — greet: smallest possible finished CLI.
//!
//! Run: `cargo run -p day-001-greet -- YourName`
//! Micro mode (no args): just prints a default greeting.

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "world".to_string());
    println!("Hello, {name}! (Day 001)");
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_name_is_world() {
        let name = Option::<String>::None.unwrap_or_else(|| "world".to_string());
        assert_eq!(name, "world");
    }
}
