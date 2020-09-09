## The stack, the heap, and pointers

The stack, the heap, and pointers are very important in Rust.

The stack and the heap are two places to keep memory. The important differences are:

- The stack is very fast, but the heap is not so fast.
- The stack needs to know the size of a variable at compile time. So simple variables like `i32` go on the stack, because we know their exact size.
- Some types don't know the size at compile time. But the stack needs to know the exact size. What do you do? You put the data in the heap, because the heap can have any size of data. And to find it, a pointer goes on the stack, because we always know the size of a pointer.

A pointer is like a table of contents in a book.

```text
MY BOOK

Chapter                        Page
Chapter 1: My life              1
Chapter 2: My cat               15
Chapter 3: My job               23
Chapter 4: My family            30
Chapter 5: Future plans         43
```

So this is like five pointers. Where is the chapter "My life"? It's on page 1 (it points to page 1). Where is the chapter "My job?" It's on page 23.

The pointer you usually see in Rust is called a **reference**. This is the important part to know: a reference points to the memory of another value. A reference means you *borrow* the value, but you don't own it. In Rust, references have a `&`. So:

- `let my_variable = 8` makes a regular variable, but
- `let my_reference = &my_variable` makes a reference.

This means that `my_reference` is only looking at the data of `my_variable`. `my_variable` still owns its data.

