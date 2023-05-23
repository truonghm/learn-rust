# Notes about variables

Read the corresponding [chapter](https://rust-book.cs.brown.edu/ch03-01-variables-and-mutability.html) of the Rust book.

## Immutability

- Make the variable mutable with the `let mut` syntax
- Otherwise, the variable is immutable by default

## Constant

- Constants are always immutable
- Constants can be used in any scope
- Constants can only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime

## Shadowing

- Declare a new variable with the same name as a previous variable -> shadowing
- When the variable's name is used, the compiler only see the new variable, until it goes out of scope or is shadowed by another variable with the same name

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```