## Comments

To write comments in Rust you usually use `//`:

```rust
fn main() { // Rust programs start with fn main()
            // You put the code inside a block. It starts with { and ends with }
    let some_number = 100; // We can write as much as we want here and the compiler won't look at it
}
```

When you do this, the compiler won't look at anything to the right of the `//`. 

There is another kind of comment that you write with `/*` to start and `*/` to end. This one is useful to write in the middle of your code.

```rust
fn main() {
    let some_number/*: i16*/ = 100;
}
```

To the compiler, both of these look the same.

The `/* */` form is also useful for very long comments over more than one line. In this example you can see that you need to write `//` for every line. But if you type `/*`, it won't stop until you finish it with `*/`.

```rust
fn main() {
    let some_number = 100; /* Let me tell you
    a little about this number.
    It's 100, which is my favourite number.
    It's called some_number but actually I think that... */

    let some_number = 100; // Let me tell you
    // a little about this number.
    // It's 100, which is my favourite number.
    // It's called some_number but actually I think that...
}
```

Later on when you create your own code and want other people to read it, you will use `///` for the comments. When Rust sees `///` it can automatically put the comments into documents to explain the code.

