## A tour of the standard library

Now that you understand a lot of Rust, you will be able to understand most things inside the standard library. Let's take a look at some of the parts in it that we haven't learned yet. This tour will go over every part of the standard library that you don't need to install Rust for.

### Arrays

One thing about arrays to note is that they don't implement `Iterator.`. That means that if you have an array, you can't use `for`. But you can use methods like `.iter()` on them. Or you can use `&` to get a slice. Actually, the compiler will tell you exactly that if you try to use `for`:

```rust
fn main() {
    // ⚠️
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];

    for city in my_cities {
        println!("{}", city);
    }
}
```

The message is:

```text
error[E0277]: `[&str; 3]` is not an iterator
 --> src\main.rs:5:17
  |
  |                 ^^^^^^^^^ borrow the array with `&` or call `.iter()` on it to iterate over it
```

So let's try both. They give the same result.

```rust
fn main() {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];

    for city in &my_cities {
        println!("{}", city);
    }
    for city in my_cities.iter() {
        println!("{}", city);
    }
}
```

This prints:

```text
Beirut
Tel Aviv
Nicosia
Beirut
Tel Aviv
Nicosia
```



If you want to get variables from an array, you can put their names inside `[]` to destructure it. This is the same as using a tuple in `match` statements or to get variables from a struct.

```rust
fn main() {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];
    let [city1, city2, city3] = my_cities;
    println!("{}", city1);
}
```

This prints `Beirut`.
 
### char

You can use the `.escape_unicode()` method to get the Unicode number for a `char`:

```rust
fn main() {
    let korean_word = "청춘예찬";
    for character in korean_word.chars() {
        print!("{} ", character.escape_unicode());
    }
}
```

This prints `\u{ccad} \u{cd98} \u{c608} \u{cc2c}`.


You can get a char from `u8` using the `From` trait, but for a `u32` you use `TryFrom` because it might not work. There are many more numbers in `u32` than characters in Unicode. We can see this with a simple demonstration.

```rust
use std::convert::TryFrom; // You need to brig TryFrom in to use it
use rand::prelude::*; // We will use random numbers too

fn main() {
    let some_character = char::from(99); // This one is easy - no need for TryFrom
    println!("{}", some_character);

    let mut random_generator = rand::thread_rng();
	// This will try 40,000 times to make a char from a u32.
        // The range is 0 (std::u32::MIN) to u32's highest number (std::u32::MAX). If it doesn't work, we will give it '-'.
    for _ in 0..40_000 {
        let bigger_character = char::try_from(random_generator.gen_range(std::u32::MIN, std::u32::MAX)).unwrap_or('-');
        print!("{}", bigger_character)
    }
}
```

Almost every time it will generate a `-`. This is part of the sort of output you will see:

```text
------------------------------------------------------------------------𤒰---------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
-------------------------------------------------------------춗--------------------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------
------------򇍜----------------------------------------------------
```

So it's a good thing you need to use `TryFrom`.


### Integers

There are a lot of math methods for these types, plus some others. Here are some of the most useful ones.



`.checked_add()`, `.checked_sub()`, `.checked_mul()`, `.checked_div()`. These are good methods if you think you might get a number that won't fit into a type. They return an `Option` so you can safely check that your math works without making the program panic.

```rust
fn main() {
    let some_number = 200_u8;
    let other_number = 200_u8;
    
    println!("{:?}", some_number.checked_add(other_number));
    println!("{:?}", some_number.checked_add(1));
}
```

This prints:

```text
None
Some(201)
```


You'll notice that on the page for integers it says `rhs` a lot. This means "right hand side", which is the right hand side when you do some math. For example, in `5 + 6`, `5` is on the left and `6` is on the right, so it's the `rhs`. This is not a keyword, but you will see it a lot so it's good to know.

While we are on the subject, let's learn how to implement `Add`. After you implement `Add`, you can use `+` on a type that you create. You need to implement `Add` yourself because add can mean a lot of things. Here's the example in the standard library page:

```rust
use std::ops::Add; // first bring in Add

#[derive(Debug, Copy, Clone, PartialEq)] // PartialEq is probably the most important part here. You want to be able to compare numbers
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self; // Remember, this is called an "associated type": a "type that goes together".
                        // In this case it's just another Point

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

Now let's implement `Add` for our own type. Maybe we want to add two countries together so we can compare their economies. It looks like this:

```rust
use std::fmt;
use std::ops::Add;

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32, // This is the size of the economy
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, other.name), // We will add the names together,
            population: self.population + other.population, // and the population, 
            gdp: self.gdp + other.gdp,   // and the GDP
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In {} are {} people and a GDP of ${}", // Then we can print them all with just {}
            self.name, self.population, self.gdp
        )
    }
}

fn main() {
    let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("Vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("Micronesia", 104_468, 367_000_000);

	// We could have given Country a &str instead of a String for the name. But we would have to write lifetimes everywhere
        // and that would be too much for a small example. Better to just clone them when we call println!.
    println!("{}", nauru.clone());
    println!("{}", nauru.clone() + vanuatu.clone());
    println!("{}", nauru + vanuatu + micronesia);
}
```

This prints:

```text
In Nauru are 10670 people and a GDP of $160000000
In Nauru and Vanuatu are 318485 people and a GDP of $980000000
In Nauru and Vanuatu and Micronesia are 422953 people and a GDP of $1347000000
```

Later on in this code we could change `.fmt()` to display a number that is easier to read.

The three others are called `Sub`, `Mul`, and `Div`, and they are basically the same to implement. For `+=`, `-=`, `*=` and `/=`, just add `Assign`: `AddAssign`, `SubAssign`, `MulAssign`, and `DivAssign`. You can see the full list [here](https://doc.rust-lang.org/std/ops/index.html#structs), because there are many more. `%` for example is called `Rem`, `-` is called `Neg`, and so on.


### Floats

`f32` and `f64` have a very large number of methods that you use when doing math. We won't look at those, but here are some methods that you might use. They are: `.floor()`, `.ceil()`, `.round()`, and `.trunc()`. All of these return an `f32` or `f64` that is like an integer, with only `0` after the period. They do this:

- `.floor()`: gives you the next lowest integer.
- `.ceil()`: gives you the next highest integer.
- `.round()`: gives you a higher number if 0.5 or more, or the same number if less than 0.5. This is called rounding because it gives you a "round" number (a number that has a short, simple form).
- `.trunc()`: just cuts off the part after the period. Truncate means "to cut off".

Here is a simple function to print them.

```rust
fn four_operations(input: f64) {
    println!(
"For the number {}:
floor: {}
ceiling: {}
rounded: {}
truncated: {}\n",
        input,
        input.floor(),
        input.ceil(),
        input.round(),
        input.trunc()
    );
}

fn main() {
    four_operations(9.1);
    four_operations(100.7);
    four_operations(-1.1);
    four_operations(-19.9);
}
```

This prints:

```text
For the number 9.1:
floor: 9
ceiling: 10
rounded: 9 // because less than 9.5
truncated: 9

For the number 100.7:
floor: 100
ceiling: 101
rounded: 101 // because more than 100.5
truncated: 100

For the number -1.1:
floor: -2
ceiling: -1
rounded: -1
truncated: -1

For the number -19.9:
floor: -20
ceiling: -19
rounded: -20
truncated: -19
```

`f32` and `f64` have a method called `.max()` and `.min()` that gives you the higher or the lower of two numbers. (For other types you can just use `std::cmp::max` and `std::cmp::min`.) Here is a way to use this with `.fold()` to get the highest or lowest number. You can see again that `.fold()` isn't just for adding numbers.

```rust
fn main() {
    let my_vec = vec![8.0_f64, 7.6, 9.4, 10.0, 22.0, 77.345, 10.22, 3.2, -7.77, -10.0];
    let maximum = my_vec.iter().fold(f64::MIN, |current_number, next_number| current_number.max(*next_number)); // Note: start with the lowest possible number for an f64.
    let minimum = my_vec.iter().fold(f64::MAX, |current_number, next_number| current_number.min(*next_number)); // And here start with the highest possible number
    println!("{}, {}", maximum, minimum);
}
```

### bool

In Rust you can turn a `bool` into an integer if you want, because it's safe to do that. But you can't do it the other way around. As you can see, `true` turns to 1 and `false` turns to 0.

```rust
fn main() {
    let true_false = (true, false);
    println!("{} {}", true_false.0 as u8, true_false.1 as i32);
}
```

This prints `1 0`. Or you can use `.into()` if you tell the compiler the type:

```rust
fn main() {
    let true_false: (i128, u16) = (true.into(), false.into());
    println!("{} {}", true_false.0, true_false.1);
}
```

This prints the same thing.


### Vec

Vec has a lot of methods that we haven't looked at yet. Let's start with `.sort()`. `.sort()` is not surprising at all. It uses a `&mut self` to sort a vector.

```rust
fn main() {
    let mut my_vec = vec![100, 90, 80, 0, 0, 0, 0, 0];
    my_vec.sort();
    println!("{:?}", my_vec);
}
```

This prints `[0, 0, 0, 0, 0, 80, 90, 100]`. But there is one more interesting way to sort called `.sort_unstable()`, and it is usually faster. It can be faster because it doesn't care about the order of numbers if they are the same number. In regular `.sort()`, you know that the last `0, 0, 0, 0, 0` will be in the same order after `.sort()`. But `.sort_unstable()` might move the last zero to index 0, then the third last zero to index 2, etc.


`.dedup()` means "de-duplicate". It will remove items that are the same in a vector, but only if they are next to each other. This next code will not just print `"sun", "moon"`:

```rust
fn main() {
    let mut my_vec = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    my_vec.dedup();
    println!("{:?}", my_vec);
}
```

It only gets rid of "sun" next to the other "sun", then "moon" next to one "moon", and again with "moon" next to another "moon". The result is: `["sun", "moon", "sun", "moon"]`.

If you want to remove every duplicate, just `.sort()` first:

```rust
fn main() {
    let mut my_vec = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    my_vec.sort();
    my_vec.dedup();
    println!("{:?}", my_vec);
}
```

Result: `["moon", "sun"]`.


### String

You will remember that a `String` is kind of like a `Vec`. It is so like a `Vec` that you can do a lot of the same methods. For example, you can start one with `String::with_capacity()`. You want that if you are always going to be pushing a `char` with `.push()` or pushing a `&str` with `.push_str()`. Here's an example of a `String` that has too many allocations.

```rust
fn main() {
    let mut push_string = String::new();
    let mut capacity_counter = 0; // capacity starts at 0
    for _ in 0..100_000 { // Do this 100,000 times
        if push_string.capacity() != capacity_counter { // First check if capacity is different now
            println!("{}", push_string.capacity()); // If it is, print it
            capacity_counter = push_string.capacity(); // then update the counter
        }
        push_string.push_str("I'm getting pushed into the string!"); // and push this in every time
    }    
}
```

This prints:

```text
35
70
140
280
560
1120
2240
4480
8960
17920
35840
71680
143360
286720
573440
1146880
2293760
4587520
```

We had to reallocate (copy everything over) 18 times. But now we know the final capacity. So we'll give it the capacity right away, and we don't need to reallocate: just one `String` capacity is enough.

```rust
fn main() {
    let mut push_string = String::with_capacity(4587520); // We know the exact number. Some different big number could work too
    let mut capacity_counter = 0;
    for _ in 0..100_000 {
        if push_string.capacity() != capacity_counter {
            println!("{}", push_string.capacity());
            capacity_counter = push_string.capacity();
        }
        push_string.push_str("I'm getting pushed into the string!");
    }    
}
```

And this prints `4587520`. Perfect! We never had to allocate again.

Of course, the actual length is certainly smaller than this. If you try 100,001 times, 101,000 times, etc., it'll still say `4587520`. That's because each time the capacity is two times what it was before. We can shrink it though with `.shrink_to_fit()` (same as for a `Vec`). Our `String` is very large and we don't want to add anything more to it, so we can make it a bit smaller. But only do this if you are sure. Here is why:

```rust
fn main() {
    let mut push_string = String::with_capacity(4587520);
    let mut capacity_counter = 0;
    for _ in 0..100_000 {
        if push_string.capacity() != capacity_counter {
            println!("{}", push_string.capacity());
            capacity_counter = push_string.capacity();
        }
        push_string.push_str("I'm getting pushed into the string!");
    } 
    push_string.shrink_to_fit();
    println!("{}", push_string.capacity());    
    push_string.push('a');
    println!("{}", push_string.capacity());    
    push_string.shrink_to_fit();
    println!("{}", push_string.capacity());  
}
```

This prints: 

```text
4587520
3500000
7000000
3500001
```

So first we had a size of `4587520`, but we weren't using it all. We used `.shrink_to_fit()` and got the size down to `3500000`. But then we forget that we needed to push an `a` on. When we did that, Rust saw that we needed more space and gave us double: now it's `7000000`. Whoops! So we did `.shrink_to_fit()` again and now it's back down to `3500001`.

`.pop()` works for a `String`, just like for a `Vec`.

```rust
fn main() {
    let mut my_string = String::from(".daer ot drah tib elttil a si gnirts sihT");
    loop {
        let pop_result = my_string.pop();
        match pop_result {
            Some(character) => print!("{}", character),
            None => break,
        }
    }
}
```

This prints `This string is a little bit hard to read.` because it starts from the last character.

`.retain()` is a method that uses a closure, which is rare for `String`. It's just like `.filter()` for an iterator.

```rust
fn main() {
    let mut my_string = String::from("Age: 20 Height: 194 Weight: 80");
    my_string.retain(|character| character.is_alphabetic() || character == ' '); // Keep if a letter or a space
    dbg!(my_string); // Let's use dbg!() for fun this time instead of println!
}
```

This prints:

```text
[src\main.rs:4] my_string = "Age  Height  Weight "
```


