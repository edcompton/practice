## Control flow

Control flow means telling your code what to do in different situations. The simplest control flow is `if`.

```rust
fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    }
}
```

Please note that we wrote `if my_number == 7` and not `if (my_number == 7)`. You don't need `()` with `if` in Rust.

`else if` and `else` gives you more control:

```rust
fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }
}
```

You can add more conditions with `&&` (and) and `||` (or).

Too much `if`, `else`, and `else if` can be difficult to read. You can use `match` instead. But you must match for every possible result. For example, this will not work:

```rust
fn main() {
    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        // ⚠️
    }
}
```

The compiler says:

```text
error[E0004]: non-exhaustive patterns: `3u8..=std::u8::MAX` not covered
 --> src\main.rs:3:11
  |
3 |     match my_number {
  |           ^^^^^^^^^ pattern `3u8..=std::u8::MAX` not covered
```

This means "you told me about 0 to 2, but u8s can go up to 255. What about 3? What about 4? What about 5?" And so on. So you can add `_` which means "anything else".

```rust
fn main() {
    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    }
}
```

Remember this for match:

- You write `match` and then make a `{}` code block.
- Write the pattern on the left and use a `=>` fat arrow to say what to do when it matches.
- Each line is called an "arm".
- Put a comma between the arms (not a semicolon).

You can declare a value with a match:

```rust
fn main() {
    let my_number = 5;
    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };
}
```

Do you see the semicolon at the end? That is because, after the match is over, we actually told the compiler this: `let second_number = 10;`

You can match on more complicated things too. You use a tuple to do it.

```rust
fn main() {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }
}
```

You can even put `if` inside of `match`.

```rust
fn main() {
    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if married == false => println!("Not married with {} children", children),
        (children, married) if children == 0 && married == true => println!("Married but no children"),
        _ => println!("Married? {}. Number of children: {}.", married, children),
    }
}
```

You can use _ as many times as you want in a match. In this match on colours, we have three but only check one at a time.

```rust
fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);

}
```

This prints:

```text
Not much blue
Each colour has at least 10
Not much green
```

This also shows how `match` statements work, because in the first example it only printed `Not much blue`. But `first` also has not much green. A `match` statement always stops when it finds a match, and doesn't check the rest. This is a good example of code that compiles well but is not the code you want. You can make a really big `match` statement to fix it, but it is probably better to use a `for` loop. We will talk about loops soon.

A match has to return the same type. So you can't do this:

```rust
fn main() {
    let my_number = 10;
    let some_variable = match my_number {
        10 => 8,
        _ => "Not ten", // ⚠️
    };
}
```

The compiler tells you that:

```text
error[E0308]: `match` arms have incompatible types
  --> src\main.rs:17:14
   |
15 |       let some_variable = match my_number {
   |  _________________________-
16 | |         10 => 8,
   | |               - this is found to be of type `{integer}`
17 | |         _ => "Not ten",
   | |              ^^^^^^^^^ expected integer, found `&str`
18 | |     };
   | |_____- `match` arms have incompatible types
```

This will also not work, for the same reason:

```rust
fn main() {
    let some_variable = if my_number == 10 { 8 } else { "something else "}; // ⚠️
}
```

But this works, because you have a different `let` statement.

```rust
fn main() {
    let my_number = 10;

    if my_number == 10 {
        let some_variable = 8;
    } else {
        let some_variable = "Something else";
    }
}
```

You can also use `@` to use the value of a `match` expression when you want to. In this example we match an `i32` input in a function. If it's 4 or 13 we want to use that number in a `println!` statement. Otherwise, we don't need to use it.

```rust
fn match_number(input: i32) {
    match input {
    number @ 4 => println!("{} is an unlucky number in China (sounds close to 死)!", number),
    number @ 13 => println!("{} is unlucky in North America, lucky in Italy! In bocca al lupo!", number),
    _ => println!("Looks like a normal number"),
    }
}

fn main() {
    match_number(50);
    match_number(13);
    match_number(4);
}
```

