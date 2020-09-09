## Enums

An `enum` is short for enumerations. They look similar to a struct, but are different. Here is the difference:

- Use a struct when you want one thing AND another thing.
- Use an enum when you want one thing OR another thing.

So structs are for **many things** together, while enums are for **many choices** together.

To declare an enum, write `enum` and use a code block with the options, separated by commas. Just like a `struct`, the last part can have a comma or not. We will create an enum called `ThingsInTheSky`:

```rust
enum ThingsInTheSky {
    Sun,
    Stars,
}

fn main() { }
```

This is an enum because you can either see the sun, **or** the stars: you have to choose one. These are called **variants**.

```rust
// create the enum with two choices
enum ThingsInTheSky {
    Sun,
    Stars,
}

// With this function we can use an i32 to create ThingsInTheSky.
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun, // Between 6 and 18 hours we can see the sun
        _ => ThingsInTheSky::Stars, // Otherwise, we can see stars
    }
}

// With this function we can match against the two choices in ThingsInTheSky.
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
    }
}

fn main() {
    let time = 8; // it's 8 o'clock
    let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
    check_skystate(&skystate); // Give it a reference so it can read the variable skystate
}
```

You can add data to an enum too.

```rust
enum ThingsInTheSky {
    Sun(String), // Now each variant has a string
    Stars(String),
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")), // Write the strings here
        _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{}", description), // Give the string the name description so we can use it
        ThingsInTheSky::Stars(n) => println!("{}", n), // Or you can name it n. Or anything else - it doesn't matter
    }
}

fn main() {
    let time = 8; // it's 8 o'clock
    let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
    check_skystate(&skystate); // Give it a reference so it can read the variable skystate
}
```


Parts of an `enum` can also be turned into an integer. Rust gives each arm of an `enum` a number that starts with 0. You can do things with it if your enum doesn't have any other data in it.

```rust
enum Season {
    Spring, // If this was Spring(String) or something it wouldn't work
    Summer,
    Autumn,
    Winter,
}

fn main() {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }
}
```

This prints:

```text
0
1
2
3
```

You can give it a different number though if you want - Rust doesn't care and can use it in the same way. Just add an `=` and your number to an arm that you want to have a number. You don't have to give all the arms a number. But if you don't, Rust will just add 1 from the arm before to give it a number.

```rust
enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

fn main() {
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 => println!("This is a good-sized star."),
            _ => println!("That star is pretty big!"),
        }
    }
    println!("What about DeadStar? It's the number {}.", DeadStar as u32);
}
```

This prints:


```text
Not the biggest star.
Not the biggest star.
This is a good-sized star.
This is a good-sized star.
What about DeadStar? It's the number 1001.
```

`DeadStar` would have been number 4, but now it's 1001. 

### Enums to use multiple types

You know that items in a `Vec`, array, etc. all need the same type (only tuples are different). But you can actually use an enum to put different types in. Imagine we want to have a `Vec` with `u32`s or `i32`s. Of course, you can make a `Vec<(u32, i32)>` (a vec with `(u32, i32)` tuples) but we only want one. So here you can use an enum. Here is a simple example:

```rust
enum Number {
    U32(u32),
    I32(i32),
}

fn main() { }
```

So there are two variants: the `U32` variant with a `u32` inside, and the `I32` variant with `i32` inside. `U32` and `I32` are just names we made. They could have been `UThirtyTwo` or `IThirtyTwo` or anything else.

Now, if we put them into a `Vec` we just have a `Vec<Number>`, and the compiler is happy. Because it's an enum, you have to pick one. We will use the `.is_positive()` method to pick. If it's `true` then we will choose `U32`, and if it's `false` then we will choose `I32`.

Now the code looks like this:

```rust
enum Number {
    U32(u32),
    I32(i32),
}

impl Number {
    fn new(number: i32) -> Number { // input number is i32
        match number.is_positive() {
            true => Number::U32(number as u32), // change it to u32 if it's positive
            false => Number::I32(number), // otherwise just give the number because it's already i32
        }
    }
}

fn main() {
    let my_vec = vec![Number::new(-800), Number::new(8)];

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's a i32 with the value {}", number),
        }
    }

}
```

This prints what we wanted to see:

```text
It's a i32 with the value -800
It's a u32 with the value 8
```

