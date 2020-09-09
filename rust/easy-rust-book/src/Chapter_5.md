## Types

### Primitive types

Rust has simple types that are called **primitive types**. We will start with integers and `char` (characters). Integers are whole numbers with no decimal point. There are two types of integers:

- Signed integers,
- Unsigned integers.

Signs means `+` (plus sign) and `-` (minus sign), so signed integers can be positive or negative (e.g. +8, -8). But unsigned integers can only be positive, because they do not have a sign.

The signed integers are: `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.
The unsigned integers are: `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

The number after the i or the u means the number of bits for the number, so numbers with more bits can be larger. 8 bits = one byte, so `i8` is one byte, `i64` is 8 bytes, and so on. Number types with larger sizes can hold larger numbers. For example, a `u8` can hold up to 255, but a `u16` can hold up to 65535. And a `u128` can hold up to 340282366920938463463374607431768211455.

So what is `isize` and `usize`? This means the number of bits on your type of computer. (This is called the **architecture** of your computer.) So `isize` and `usize` on a 32-bit computer is like `i32` and `u32`, and `isize` and `usize` on a 64-bit computer is like `i64` and `u64`.

There are many reasons for the different types of integers. One reason is computer performance: a smaller number of bytes is faster to process. But here are some other uses:

Characters in Rust are called `char`. Every `char` has a number: the letter `A` is number 65, while the character `友` ("friend" in Chinese) is number 21451. The list of numbers is called "Unicode". Unicode uses smaller numbers for characters that are used more, like A through Z, or digits 0 through 9, or space. 

```rust
fn main() {
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are characters too
}
```

The characters that are used most get numbers that are less than 256, and they can fit into a `u8`. This means that Rust can safely **cast** a `u8` into a `char`, using `as`. (Cast `u8` as `char` means "pretend `u8` is a `char`")

Casting with `as` is useful because Rust is very strict. It always needs to know the type, and won't let you use two different types together even if they are both integers. For example, this will not work:

```rust
fn main() { // main() is where Rust programs start to run. Code goes inside {} (curly brackets)

    let my_number = 100; // We didn't write a type of integer,
                         // so Rust chooses i32. Rust always
                         // chooses i32 for integers if you don't
                         // tell it to use a different type
			 
    println!("{}", my_number as char); // ⚠️
}
```

Here is the reason:

```text
error[E0604]: only `u8` can be cast as `char`, not `i32`
 --> src\main.rs:3:20
  |
3 |     println!("{}", my_number as char);
  |                    ^^^^^^^^^^^^^^^^^
```

Fortunately we can easily fix this with `as`. We can't make `i32` a `char`, but we can make a `i32` a `u8`. And then we can make `u8` a `char`. So in one line we use `as` to make my_number a `u8`, and once more to make it a `char`. Now it will compile:

```rust
fn main() {
    let my_number = 100;
    println!("{}", my_number as u8 as char);
}
```

Here is another reason for the different sizes: `usize` is the size that Rust uses for *indexing*. (Indexing means "which item is first", "which item is second", etc.) `usize` is the best size for indexing because:

- An index can't be negative, so it needs to be a number with a u
- It should be big, because sometimes you need to index many things, but
- It can't be a u64 because 32-bit computers can't use that.

So Rust uses `usize` so that your computer can get the biggest number for indexing that it can read.


Let's learn some more about `char`. You saw that a `char` is always one character, and uses `''` instead of `""`.

All chars are 4 bytes. They are 4 bytes because some characters in a string are more than one byte. Basic letters that have always been on computers are 1 byte, later characters are 2 bytes, and others are 3 and 4. A `char` is 4 bytes so that it can fit any of these.

For example:

```rust
fn main() {
    println!("{}", "a".len()); // .len() gives the size in bytes
    println!("{}", "ß".len());
    println!("{}", "国".len());
    println!("{}", "𓅱".len());
}
```

This prints:

```text
1
2
3
4
```

You can see that `a` is one byte, the German `ß` is two, the Japanese `国` is three, and the ancient Egyptian `𓅱` is 4 bytes.

```rust
fn main() {
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());
}
```

`slice` is six characters in length and six bytes, but `slice2` is three characters in length and seven bytes. `char` needs to fit any character in any language, so it is 4 bytes long.

If `.len()` gives the size in bytes, what about the size in characters? We will learn about these methods later, but you can just remember that `.chars().count()` will do it.


```rust
fn main() {
    let slice = "Hello!";
    println!("Slice is {} characters.", slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} characters.", slice2.chars().count());
}
```

This prints:

```text
Slice is 6 characters.
Slice2 is 3 characters.
```

