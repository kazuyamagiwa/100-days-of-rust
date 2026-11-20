//! Day 001 — greet
//!
//! A *program* is a list of instructions the computer follows, top to bottom.
//! This file is the whole program. Run it with:
//!   cargo run -p day-001-greet
//!   cargo run -p day-001-greet -- YourName
//! (`cargo` = Rust's helper tool; it builds then runs. `--` passes YourName into our program.)

// `fn` = function (a named chunk of steps). `main` is special: Rust starts here.
fn main() {
    // `let name = ...` creates a label called `name` and stores a value in it.
    // Think: sticky note on a box. The box holds text (a person's name).
    let name = std::env::args()
        // When you type a command, extra words after it are "arguments".
        // args() gives them as a list. Index 0 is the program name itself.
        // .nth(1) = "give me item number 1" (the first word *you* typed), or nothing.
        .nth(1)
        // "Nothing" is written `None`; "something" is `Some(value)`.
        // unwrap_or_else: if we got None, run the tiny function on the right
        // and use `"world"` instead. So a bare run still greets someone.
        .unwrap_or_else(|| "world".to_string());
    // `to_string()` turns the fixed text `"world"` into an owned String (editable text in memory).

    // `println!` prints a line to the screen. `{name}` inserts the value of `name`.
    // The `!` means this is a macro (a special code shortcut). You can treat it like a function for now.
    println!("Hello, {name}! (Day 001)");
}

// --- Tests (optional on micro days) ---
// A test is a tiny check: "does this bit behave as I expect?"
// Run with: cargo test -p day-001-greet
#[cfg(test)] // only compile this block when testing
mod tests {
    #[test]
    fn default_name_is_world() {
        // Same idea as above: if there is no name, we want "world".
        let name = Option::<String>::None.unwrap_or_else(|| "world".to_string());
        // assert_eq!: fail the test if the two sides are not equal.
        assert_eq!(name, "world");
    }
}
