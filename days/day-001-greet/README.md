# Day 001 — greet

Tiny CLI: print a greeting. Optional name from the command line.

Open `src/main.rs` — comments explain the basics (program, `main`, variables, arguments). Skim ~1–2 min, or just run on micro days.

```bash
cargo run -p day-001-greet
cargo run -p day-001-greet -- YourName
cargo test -p day-001-greet
```

**LinkedIn post (copy):**  
Day 1 of #100DaysOfRust — shipped a tiny greet CLI. Learning in public with small, finished programs (done > impressive). Today: reading args via `std::env`.

**Touched Rust today:** `fn main`, `let`, `std::env::args`, `unwrap_or_else`, `println!`.