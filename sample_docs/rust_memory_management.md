# Rust Memory Management: Ownership and Borrowing

Rust's defining feature is memory safety without a garbage collector. It achieves this through a system of ownership with a set of rules that the compiler checks at compile time.

## Ownership Rules
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Borrowing
Instead of transferring ownership, you can borrow a reference to a value. 
- You can have either one mutable reference or any number of immutable references.
- References must always be valid (no dangling pointers).

This ensures data races are impossible at compile time!
