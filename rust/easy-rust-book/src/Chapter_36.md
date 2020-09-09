## Traits

We have seen traits before: Debug, Copy, Clone are all traits. To give a type a trait, you have to implement it. Because Debug and the others are so common, it's easy to do:

```rust
#[derive(Debug)]
struct MyStruct {
    number: usize,
}

fn main() { }
```

But other traits are more difficult, so you need to implement them manually with `impl`. For example, std::ops::Add is used to add two things. But you can add in many ways.

```rust
struct ThingsToAdd {
    first_thing: u32,
    second_thing: f32,
}

fn main() { }
```

We can add `first_thing` and `second_thing`, but we need to give more information. Maybe we want an `f32`, so something like this:

```rust
// 🚧
let result = self.second_thing + self.first_thing as f32
```

But maybe we want an integer, so like this:

```rust
// 🚧
let result = self.second_thing as u32 + self.first_thing
```

Or maybe we want to just put `self.first_thing` next to `self.second_thing` and say that this is how we want to add. So if we add 55 to 33.4, we want to see 5533, not 88.

So first let's look at how to make a trait. To make a trait, write `trait` and then create some functions.

```rust
struct Animal { // A simple struct - an Animal only has a name
    name: String,
}

trait Dog { // The dog trait gives some functionality
    fn bark(&self) { // It can bark
        println!("Woof woof!");
    }
    fn run(&self) { // and it can run
        println!("The dog is running!");
    }
}

impl Dog for Animal {} // Now Animal has the trait Dog

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark(); // Now Animal can use bark()
    rover.run();  // and it can use run()
}
```

This is okay, but we don't want to print "The dog is running". We can change the method `.run()`, but we have to follow the signature. The signature says:

```rust
// 🚧
fn run(&self) {
    println!("The dog is running!");
}
```

The signature says "fn `run()` takes `&self`, and returns nothing". So you can't do this:

```rust
fn run(&self) -> i32 { // ⚠️
    5
}
```

Rust will say:

```text
   = note: expected fn pointer `fn(&Animal)`
              found fn pointer `fn(&Animal) -> i32`
```

But we can do this:

```rust
struct Animal { // A simple struct - an Animal only has a name
    name: String,
}

trait Dog { // The dog trait gives some functionality
    fn bark(&self) { // It can bark
        println!("Woof woof!");
    }
    fn run(&self) { // and it can run
        println!("The dog is running!");
    }
}

impl Dog for Animal {
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() { }
```

Now it prints `Rover is running!`. This is okay because we are returning `()`, or nothing, which is what the trait says.

Actually, a trait doesn't need to write out the whole function. Now we change `bark()` and `run()` to just say `fn bark(&self)` and `fn run(&self);`. This is not a full function, so the user must write it.

```rust
struct Animal {
    name: String,
}

trait Dog {
    fn bark(&self); // bark() says it needs a &self and returns nothing
    fn run(&self); // run() says it needs a &self and returns nothing.
                   // So now we have to write them ourselves.
}

impl Dog for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
}
```

So when you create a trait, you must think: "Which functions should I write? And which functions should the user write?" If you think the user will use the function the same way every time, then write out the function. If you think the user will use it differently, then just write the function signature.

So let's try implementing the Display trait for our struct. First we will make a simple struct:

```rust
struct Cat {
    name: String,
    age: u8,
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
}
```

Now we want to print `mr_mantle`. Debug is easy to derive:

```rust
#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("Mr. Mantle is a {:?}", mr_mantle);
}
```

but Debug print is not what we want.

```text
Mr. Mantle is a Cat { name: "Reggie Mantle", age: 4 }
```

So we need to implement Display for Cat. On [https://doc.rust-lang.org/std/fmt/trait.Display.html](https://doc.rust-lang.org/std/fmt/trait.Display.html) we can see the information for Display, and one example. It says:

```rust
use std::fmt;

struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

fn main() { }
```

Some parts of this we don't understand yet, like `<'_>` and what `f` is doing. But we understand the `Position` struct: it is just two `f32`s. We also understand that `self.longitude` and `self.latitude` are the values in the struct. So maybe we can just use this for our struct, with `self.name` and `self.age`. Also, `write!` looks a lot like `println!`. So we write this:

```rust
use std::fmt;

struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}

fn main() { }
```

Now our code looks like this:

```rust
use std::fmt;
struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} is a cat who is {} years old.", self.name, self.age)
  }
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("{}", mr_mantle);
}
```

Success! Now when we use `{}` to print, we get `Reggie Mantle is a cat who is 4 years old.`. This looks much better.

### The From trait

*From* is a very convenient trait to use, and you know this because you have seen it so much already. With *From* you can make a `String` from a `&str`, but you can make many types from many other types. For example, Vec uses *From* for the following:

```text
From<&'_ [T]>
From<&'_ mut [T]>
From<&'_ str>
From<&'a Vec<T>>
From<[T; N]>
From<BinaryHeap<T>>
From<Box<[T]>>
From<CString>
From<Cow<'a, [T]>>
From<String>
From<Vec<NonZeroU8>>
From<Vec<T>>
From<VecDeque<T>>
```

That is a lot of `Vec::from()` that we have not tried yet. Let's make a few and see what happens.

```rust
use std::fmt::Display; // We will make a generic function to print them so we want Display

fn print_vec<T: Display>(input: &Vec<T>) { // Take any Vec<T> if type T has Display
    for item in input {
        print!("{} ", item);
    }
    println!();
}

fn main() {

    let array_vec = Vec::from([8, 9, 10]); // Try from an array
    print_vec(&array_vec);

    let str_vec = Vec::from("What kind of vec will I be?"); // An array from a &str? This will be interesting
    print_vec(&str_vec);

    let string_vec = Vec::from("What kind of vec will a String be?".to_string()); // Also from a String
    print_vec(&string_vec);
}
```

It prints the following:

```text
8 9 10
87 104 97 116 32 107 105 110 100 32 111 102 32 118 101 99 32 119 105 108 108 32 73 32 98 101 63
87 104 97 116 32 107 105 110 100 32 111 102 32 118 101 99 32 119 105 108 108 32 97 32 83 116 114
105 110 103 32 98 101 63
```

If you look at the type, the second and third vectors are `Vec<u8>`, which means the bytes of the `&str` and the `String`. So you can see that `From` is very flexible and used a lot.

Let's make two structs and then implement `From` for one of them. One struct will be `City`, and the other will be `Country`. We want to be able to do this: `let country_name = Country::from(vector_of_cities)`.

It looks like this:

```rust
#[derive(Debug)] // So we can print City
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self { // just a new function
        Self {
            name: name.to_string(),
            population,
        }
    }
}
#[derive(Debug)] // Country also needs to be printed
struct Country {
    cities: Vec<City>, // Our cities go in here
}

impl From<Vec<City>> for Country { // Note: we don't have to write From<City>, we can also do
                                   // From<Vec<City>>. So we can also implement on a type that
                                   // we didn't create
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) { // function to print the cities in Country
        for city in &self.cities {
            // & because Vec<Cities> isn't Copy
            println!("{:?} has a population of {:?}.", city.name, city.population);
        }
    }
}

fn main() {
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku]; // This is the Vec<City>
    let finland = Country::from(finland_cities); // So now we can use From

    finland.print_cities();
}
```

You can see that `From` is easy to implement from types you didn't create like `Vec`, `i32`, and so on. Here is one more example where we create a vector that has two vectors. The first vector holds even numbers, and the second holds odd numbers. With `From` you can give it a vector of `i32`s and it will turn it into a `Vec<Vec<i32>>`: a vector that holds vectors of `i32`.

```rust
use std::convert::From;

#[derive(Debug)]
struct EvenOddVec(Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
    fn from(input: Vec<i32>) -> Self {
        let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]]; // A vec with two empty vecs inside
                                                                    // This is the return value but first we must fill it
        for item in input {
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }
        Self(even_odd_vec) // Now it is done so we return it as Self (Self = EvenOddVec)
    }
}

fn main() {
    let bunch_of_numbers = vec![8, 7, -1, 3, 222, 9787, -47, 77, 0, 55, 7, 8];
    let new_vec = EvenOddVec::from(bunch_of_numbers);

    println!("{:?}", new_vec);
}
```

A type like `EvenOddVec` is probably better as a generic `T` so we can use many number types. You can try to make the example generic if you want for practice.

### Taking a String and a &str in a function

Sometimes you want a function that can take both a `String` and a `&str`. You can do this with generics and the `AsRef` trait. `AsRef` is used to give a reference from one type to another type. If you look at the documentation for `String`, you can see that it has `AsRef` for many types:

[https://doc.rust-lang.org/std/string/struct.String.html](https://doc.rust-lang.org/std/string/struct.String.html)

Here are some function signatures for them.

`AsRef<str>`:

```rust
// 🚧
impl AsRef<str> for String

fn as_ref(&self) -> &str
```

`AsRef<[u8]>`:

```rust
// 🚧
impl AsRef<[u8]> for String

fn as_ref(&self) -> &[u8]
```

`AsRef<OsStr>`:

```rust
// 🚧
impl AsRef<OsStr> for String

fn as_ref(&self) -> &OsStr
```

You can see that it takes `&self` and gives a reference to the other type. This means that if you have a generic type T, you can say that it needs `AsRef<str>`. If you do that, it will be able to take a `&str` and a `String`.

Let's start with the generic function. This doesn't work yet:

```rust
fn print_it<T>(input: T) {
    println!("{}", input) // ⚠️
}

fn main() {
    print_it("Please print me");
}
```

Rust says `error[E0277]: T doesn't implement std::fmt::Display`. So we will require T to implement Display.

```rust
use std::fmt::Display;

fn print_it<T: Display>(input: T) {
    println!("{}", input)
}

fn main() {
    print_it("Please print me");
}
```

Now it works and prints `Please print me`. That is good, but T can still be too many things. It can be an `i8`, an `f32` and anything else with just `Display`. So we add `AsRef<str>`, and now T needs both `AsRef<str>` and `Display`.

```rust
use std::fmt::Display;

fn print_it<T: AsRef<str> + Display>(input: T) {
    println!("{}", input)
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
    // print_it(7); <- This will not print
}
```

Now it won't take types like `i8`.

Don't forget that you can write the function differently when it gets long. If we add Debug then it becomes `fn print_it<T: AsRef<str> + Display + Debug>(input: T)` which is long for one line. So we can write it like this:

```rust
use std::fmt::{Debug, Display}; // add Debug

fn print_it<T>(input: T) // Now this line is easy to read
where
    T: AsRef<str> + Debug + Display, // and these traits are easy to read
{
    println!("{}", input)
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
```

