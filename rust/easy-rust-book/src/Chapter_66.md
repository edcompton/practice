## OsString and CString

`std::ffi` is the part of the standard library that helps you use Rust with other languages or operating systems. It has types like `OsString` and `CString`, which are like `String` for the operating system or if you need to work with the language C. They each have their own `&str` type too: `OsStr` and `CStr`. `ffi` means "foreign function interface".

`OsString` is necessary because sometimes you have to work with an operating system that doesn't have Unicode. All Rust strings are unicode, but not every operating system has it. Here is the simple English explanation from the standard library why `OsString` is necessary.

- A string on Unix (Linux, etc.) might be lots of bytes together that don't have zeros. And sometimes you read them as Unicode UTF-8.
- A string on Windows might be made of random 16-bit values that don't have zeros. And sometimes you read them as Unicode UTF-16.
- In Rust, strings are always valid UTF-8, which may contain zeros.

So an `OsString` is made to be read by all of them.

You can do all the regular things with an `OsString` like `OsString::from("Write something here")`. It also has an interesting method called `.into_string()` that tries to make it into a regular `String`. It returns a `Result`, but the `Err` part is just the original `OsString`:

```rust
// 🚧
pub fn into_string(self) -> Result<String, OsString>
```

So if it doesn't work then you just get it back. You can't call `.unwrap()` as it will panic, but you can use `match` to get the `OsString` back. Let's test it out by calling methods that don't exist.

```rust
use std::ffi::OsString;

fn main() {
    // ⚠️
    let os_string = OsString::from("This string works for your OS too.");
    match os_string.into_string() {
        Ok(valid) => valid.thth(),           // Compiler: "What's .thth()??"
        Err(not_valid) => not_valid.occg(),  // Compiler: "What's .occg()??"
    }
}
```

Then the compiler tells us exactly what we want to know:

```text
error[E0599]: no method named `thth` found for struct `std::string::String` in the current scope
 --> src/main.rs:6:28
  |
6 |         Ok(valid) => valid.thth(),
  |                            ^^^^ method not found in `std::string::String`

error[E0599]: no method named `occg` found for struct `std::ffi::OsString` in the current scope
 --> src/main.rs:7:37
  |
7 |         Err(not_valid) => not_valid.occg(),
  |                                     ^^^^ method not found in `std::ffi::OsString`
```

We can see that the type of `valid` is `String` and the type of `not_valid` is `OsString`.

### mem

`std::mem` has some pretty interesting methods. We saw some of them already, such as `.size_of()`, `.size_of_val()` and `.drop()`:


```rust
use std::mem;

fn main() {
    println!("{}", mem::size_of::<i32>());
    let my_array = [8; 50];
    println!("{}", mem::size_of_val(&my_array));
    let mut some_string = String::from("You can drop a String because it's on the heap");
    mem::drop(some_string);
    // some_string.clear();   If we did this it would panic
}
```

This prints:

```text
4
200
```

Here are some other methods in `mem`:

`swap()`: with this you can change the values between two variables. You use a mutable reference for each to do it. This is helpful when you have two things you want to switch and Rust doesn't let you because of borrowing rules. Or just when you want to quickly switch two things.

Here's one example:

```rust
use std::{mem, fmt};

struct Ring { // Create a ring from Lord of the Rings
    owner: String,
    former_owner: String,
    seeker: String, // seeker means "person looking for it"
}

impl Ring {
    fn new(owner: &str, former_owner: &str, seeker: &str) -> Self {
        Self {
            owner: owner.to_string(),
            former_owner: former_owner.to_string(),
            seeker: seeker.to_string(),
        }
    }
}

impl fmt::Display for Ring { // Display to show who has it and who wants it
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} has the ring, {} used to have it, and {} wants it", self.owner, self.former_owner, self.seeker)
        }
}

fn main() {
    let mut one_ring = Ring::new("Frodo", "Gollum", "Sauron"); // 
    println!("{}", one_ring);
    mem::swap(&mut one_ring.owner, &mut one_ring.former_owner); // Gollum got the ring back for a second
    println!("{}", one_ring);
}
```

This will print:

```text
Frodo has the ring, Gollum used to have it, and Sauron wants it
Gollum has the ring, Frodo used to have it, and Sauron wants it
```

`replace()`: this is like swap, and actually uses swap inside it, as you can see:

```rust
pub fn replace<T>(dest: &mut T, mut src: T) -> T {
    swap(dest, &mut src);
    src
}
```

So it just does a swap and then returns the other item. With this you replace the value with something else you put in. And since it returns the old value, so you should use it with `let`. Here's a quick example.

```rust
use std::mem;

struct City {
    name: String,
}

impl City {
    fn change_name(&mut self, name: &str) {
        let old_name = mem::replace(&mut self.name, name.to_string());
        println!(
            "The city once called {} is now called {}.",
            old_name, self.name
        );
    }
}

fn main() {
    let mut capital_city = City {
        name: "Constantinople".to_string(),
    };
    capital_city.change_name("Istanbul");
}
```

This prints `The city once called Constantinople is now called Istanbul.`.

One function called `.take()` is like `.replace()` but it leaves the default value in the item. You will remember that default values are usually things like 0, "", and so on. Here is the signature:

```rust
// 🚧
pub fn take<T>(dest: &mut T) -> T 
where
    T: Default, 
```

So you can do something like this:

```rust
use std::mem;

fn main() {
    let mut number_vec = vec![8, 7, 0, 2, 49, 9999];
    let mut new_vec = vec![];

    number_vec.iter_mut().for_each(|number| {
        let taker = mem::take(number);
        new_vec.push(taker);
    });

    println!("{:?}\n{:?}", number_vec, new_vec);
}
```

And as you can see, it replaced all the numbers with 0: no index was deleted.

```text
[0, 0, 0, 0, 0, 0]
[8, 7, 0, 2, 49, 9999]
```


Of course, for your own type you can implement `Default` to whatever you want. Let's look at an example where we have a `Bank` and a `Robber`. Every time he robs the `Bank`, he gets the money at the desk. But the desk can take money from the back any time, so it always has 50. We will make our own type for this so it will always have 50. Here is how it works:

```rust
use std::mem;
use std::ops::{Deref, DerefMut}; // We will use this to get the power of u32

struct Bank {
    money_inside: u32,
    money_at_desk: DeskMoney, // This is our "smart pointer" type. It has its own default, but it will use u32
}

struct DeskMoney(u32);

impl Default for DeskMoney {
    fn default() -> Self {
        Self(50) // default is always 50, not 0
    }
}

impl Deref for DeskMoney { // With this we can access the u32 using *
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DeskMoney { // And with this we can add, subtract, etc.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Bank {
    fn check_money(&self) {
        println!(
            "There is ${} in the back and ${} at the desk.\n",
            self.money_inside, *self.money_at_desk // Use * so we can just print the u32
        );
    }
}

struct Robber {
    money_in_pocket: u32,
}

impl Robber {
    fn check_money(&self) {
        println!("The robber has ${} right now.\n", self.money_in_pocket);
    }

    fn rob_bank(&mut self, bank: &mut Bank) {
        let new_money = mem::take(&mut bank.money_at_desk); // Here it takes the money, and leaves 50 because that is the default
        self.money_in_pocket += *new_money; // Use * because we can only add u32. DeskMoney can't add
        bank.money_inside -= *new_money;    // Same here
        println!("She robbed the bank. She now has ${}!\n", self.money_in_pocket);
    }
}

fn main() {
    let mut bank_of_klezkavania = Bank { // Set up our bank
        money_inside: 5000,
        money_at_desk: DeskMoney(50),
    };
    bank_of_klezkavania.check_money();

    let mut robber = Robber { // Set up our robber
        money_in_pocket: 50,
    };
    robber.check_money();

    robber.rob_bank(&mut bank_of_klezkavania); // Rob, then check money
    robber.check_money();
    bank_of_klezkavania.check_money();

    robber.rob_bank(&mut bank_of_klezkavania); // Do it again
    robber.check_money();
    bank_of_klezkavania.check_money();

}
```

This will print:

```text
There is $5000 in the back and $50 at the desk.

The robber has $50 right now.

She robbed the bank. She now has $100!

The robber has $100 right now.

There is $4950 in the back and $50 at the desk.

She robbed the bank. She now has $150!

The robber has $150 right now.

There is $4900 in the back and $50 at the desk.
```

You can see that there is always $50 at the desk.


### prelude

The standard library has a prelude too, which is why you don't have to write things like `use std::vec::Vec` to create a `Vec`. You can see all the items [here](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents), and will already know almost all of them:

- `std::marker::{Copy, Send, Sized, Sync, Unpin}`. You haven't seen `Unpin` before, because it is used for almost every type (like `Sized`, which is also very common). To "pin" means to not let something move. In this case a `Pin` means that it can't move in memory, but most items have `Unpin` so you can. That's why functions like `std::mem::replace` work, because they aren't pinned.
- `std::ops::{Drop, Fn, FnMut, FnOnce}`.
- `std::mem::drop`
- `std::boxed::Box`.
- `std::borrow::ToOwned`. You saw this before a bit with `Cow`, which can take borrowed content and make it owned. It uses `.to_owned()` to do this. You can also use `.to_owned()` on a `&str` to get a `String`, and the same for other borrowed values.
- `std::clone::Clone`
- `std::cmp::{PartialEq, PartialOrd, Eq, Ord}`.
- `std::convert::{AsRef, AsMut, Into, From}`.
- `std::default::Default`.
- `std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}`. We used `.rev()` for an iterator before: this actually makes a `DoubleEndedIterator`. An `ExactSizeIterator` is just something like `0..10`: it already knows that it has a `.len()` of 10. Other iterators don't know their length for sure.
- `std::option::Option::{self, Some, None}`.
- `std::result::Result::{self, Ok, Err}`.
- `std::string::{String, ToString}`.
- `std::vec::Vec`.

What if you don't want the prelude for some reason? Just add the attribute `#![no_implicit_prelude]`. Let's give it a try and watch things break:

```rust
	// ⚠️
#![no_implicit_prelude]
fn main() {
    let my_vec = vec![8, 9, 10];
    let my_string = String::from("This won't work");
    println!("{:?}, {}", my_vec, my_string);
}
```

Now Rust has no idea what you are trying to do:

```text
error: cannot find macro `println` in this scope
 --> src/main.rs:5:5
  |
5 |     println!("{:?}, {}", my_vec, my_string);
  |     ^^^^^^^

error: cannot find macro `vec` in this scope
 --> src/main.rs:3:18
  |
3 |     let my_vec = vec![8, 9, 10];
  |                  ^^^

error[E0433]: failed to resolve: use of undeclared type or module `String`
 --> src/main.rs:4:21
  |
4 |     let my_string = String::from("This won't work");
  |                     ^^^^^^ use of undeclared type or module `String`

error: aborting due to 3 previous errors
```

So to do this simple code you need to tell Rust to bring in the `extern` (external) crate called `std`, and then the items you want. Just to create a Vec and a String and to print it we have to do this:

```rust
#![no_implicit_prelude]

extern crate std; // Now you have to tell Rust that you want to use a crate called std
use std::vec; // We need the vec macro
use std::string::String; // and string
use std::convert::From; // and this to convert from a &str to the String
use std::println; // and this to print

fn main() {
    let my_vec = vec![8, 9, 10];
    let my_string = String::from("This won't work");
    println!("{:?}, {}", my_vec, my_string);
}
```

And now it finally works, printing `[8, 9, 10], This won't work`. So you can see why Rust uses the prelude. But if you want, you don't need to use it. And you can even use `#![no_std]` (we saw this once) for when you can't even use something like stack memory. But most of the time you don't have to think about not using the prelude or `std` at all.

So why didn't we see the `extern` keyword before? It's because you don't need it that much anymore. Before, when bringing in an external crate you had to use it. So to use `rand` in the past, you had to write:

```rust
extern crate rand;
```

and then `use` statements for the mods, traits, etc. that you wanted to use. But the Rust compiler now doesn't need this help anymore - you can just use `use` and it knows where to find it. So you almost never need `extern crate` anymore, but in other people's Rust code you might still see it on the top.



### time

`std::time` is where you can get functions for time. (If you want even more functions, a crate like `chrono` can work.) The simplest function is just getting the system time with `Instant::now()`.

```rust
use std::time::Instant;

fn main() {
    let time = Instant::now();
    println!("{:?}", time);
}
```

If you print it, you'll get something like this: `Instant { tv_sec: 2738771, tv_nsec: 685628140 }`. That's talking about seconds and nanoseconds, but it's not very useful. If you look at 2738771 seconds for example (written in August), it is 31.70 days. That doesn't have anything to do with the month or the day of the year. But the page on `Instant` tells us that it isn't supposed to be useful on its own. It says that it is "opaque and useful only with Duration." Opaque means "you can't figure it out", and duration means "how much time passed". So it's only useful when doing things like comparing times.

If you look at the traits on the left, one of them is `Sub<Instant>`. That means we can use `-` to subtract one from another. And when we click on [src] to see what it does, it says:

```rust
impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}
```

So it takes an `Instant` and uses `.duration_since()` to give a `Duration`. Let's try printing that. We'll make two `Instant::now()`s right next to each other, then we'll make the program busy for a while. Then we'll make one more `Instant::now()`. Finally, we'll see how long it took.

```rust
use std::time::Instant;

fn main() {
    let time1 = Instant::now();
    let time2 = Instant::now(); // These two are right next to each other

    let mut new_string = String::new();
    loop {
        new_string.push('წ'); // Make Rust push this Georgian letter onto the String
        if new_string.len() > 100_000 { //  until it is 100,000 bytes long
            break;
        }
    }
    let time3 = Instant::now();
    println!("{:?}", time2 - time1);
    println!("{:?}", time3 - time1);
}
```

This will print something like this:

```text
1.025µs
683.378µs
```

So that's just over 1 microsecond vs. 683 milliseconds. We can see that Rust did take some time to do it.

There is one fun thing we can do with just a single `Instant` though. We can turn it into a `String` with `format!("{:?}", Instant::now());`. It looks like this:

```rust
use std::time::Instant;

fn main() {
    let time1 = format!("{:?}", Instant::now());
    println!("{}", time1);
}
```

That prints something like `Instant { tv_sec: 2740773, tv_nsec: 632821036 }`. That's not useful, but if we use `.iter()` and `.rev()` and `.skip(2)`, we can skip the `}` and ` ` at the end. We can use it to make a random number generator.

```rust
use std::time::Instant;

fn bad_random_number(digits: usize) {
    if digits > 9 {
        panic!("Random number can only be up to 9 digits");
    }
    let now = Instant::now();
    let output = format!("{:?}", now);

    output
        .chars()
        .rev()
        .skip(2)
        .take(digits)
        .for_each(|character| print!("{}", character));
    println!();
}

fn main() {
    bad_random_number(1);
    bad_random_number(1);
    bad_random_number(3);
    bad_random_number(3);
}
```

This will print something like:

```text
6
4
967
180
```

The function is called `bad_random_number` because this is not a good example of a random number generator. Rust has better crates that make random numbers with less code than `rand` like `fastrand`. But it's a good example of how you can use your imagination to do something with `Instant`.

When you have a thread, you can use `std::thread::sleep` to make it stop for a while. When you do this, you have to give it a duration. You don't have to make more than one thread to do this because every program is on at least one thread. `sleep` needs a `Duration` though, so it can know how long to sleep. You can pick the unit like this: `Duration::from_millis()`, `Duration::from_secs`, etc. Here's one example:

```rust
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let three_seconds = Duration::from_secs(3);
    println!("I must sleep now.");
    sleep(three_seconds);
    println!("Did I miss anything?");
}
```

This will just print

```text
I must sleep now.
Did I miss anything?
```

but the thread will do nothing for three seconds. You usually use `.sleep()` when you have many threads that need to try something a lot, like connecting. You don't want the thread to use your whole processor to try 100,000 times in a second when you just want it to check sometimes. So then you can set a `Duration`, and it will try to do its task every time it wakes up.


### Other macros


Let's take a look at some other macros.

`unreachable!()`

This macro is kind of like `todo!()` except it's for code that you will never do. Maybe you have a `match` in an enum that you know will never choose one of the arms, so the code can never be reached. If that's so, you can write `unreachable!()` so the compiler knows that it can ignore that part.

For example, let's say you have a program that writes something when you choose a place to live in. They are in Ukraine, and all of them are nice except Chernobyl. Your program doesn't let anyone choose Chernobyl, because it's not a good place to live right now. But the enum was made a long time ago in someone else's code, and you can't change it. So in the `match` arm you can use the macro here. It looks like this:

```rust
enum UkrainePlaces {
    Kiev,
    Kharkiv,
    Chernobyl, // Pretend we can't change the enum - Chernobyl will always be here
    Odesa,
    Dnipro,
}

fn choose_city(place: &UkrainePlaces) {
    use UkrainePlaces::*;
    match place {
        Kiev => println!("You will live in Kiev"),
        Kharkiv => println!("You will live in Kharkiv"),
        Chernobyl => unreachable!(),
        Odesa => println!("You will live in Odesa"),
        Dnipro => println!("You will live in Dnipro"),
    }
}

fn main() {
    let user_input = UkrainePlaces::Kiev; // Pretend the user input is made from some other function. The user can't choose Chernobyl, no matter what
    choose_city(&user_input);
}
```

This will print `You will live in Kiev`.

`unreachable!()` is also nice for the programmer because it reminds you that some part of the code is unreachable. And of course, if it's not unreachable and the compiler calls `unreachable!()`, the program will panic.

Also, if you ever have unreachable code that the compiler knows about, it will tell you. Here is a quick example:

```rust
fn main() {
    let true_or_false = true;

    match true_or_false {
        true => println!("It's true"),
        false => println!("It's false"),
        true => println!("It's true"), // Whoops, we wrote true again
    }
}
```

It will say:

```text
warning: unreachable pattern
 --> src/main.rs:7:9
  |
7 |         true => println!("It's true"),
  |         ^^^^
  |
```

But `unreachable!()` is for when the compiler can't know, like our other example.



`column!`, `line!`, `file!`, `module_path!`

These four macros are kind of like `dbg!()` because you just put them in to give you debug information. But they don't take any variables - you just use them with the brackets and nothing else. They are easy to learn together:

- `column!()` gives you the column where you wrote it,
- `line!()` gives you the line where you wrote it, and
- `module_path!()` gives you the module where it is.

The next code shows all three in a simple example. We will pretend there is a lot more code (mods inside mods), because that is why we would want to use these macros. You can imagine a big Rust program over many mods and files.

```rust
pub mod something {
    pub mod third_mod {
        pub fn print_a_country(input: &mut Vec<&str>) {
            println!(
                "The last country is {} inside the module {}",
                input.pop().unwrap(),
                module_path!()
            );
        }
    }
}

fn main() {
    use something::third_mod::*;
    let mut country_vec = vec!["Portugal", "Czechia", "Finland"];

    // do some stuff
    println!(
        "On line {} we got the country {}",
        line!(),
        country_vec.pop().unwrap()
    );

    // do some more stuff

    println!(
        "The next country is {} on line {} and column {}.",
        country_vec.pop().unwrap(),
        line!(),
        column!(),
    );

    // lots more code

    print_a_country(&mut country_vec);
}
```

It prints this:

```text
On line 20 we got the country Finland
The next country is Czechia on line 29 and column 9.
The last country is Portugal inside the module rust_book::something::third_mod
```



`cfg!`

You will remember that you can use attributes like `#[cfg(test)]` and `#[cfg(windows)]` to tell the compiler what to do in certain cases. When you have `test`, it will run the code when you run Rust under testing mode (if it's on your computer you type `cargo test`). And when you use `windows`, it will run the code if the user is using Windows. But maybe you just want to change one tiny bit of code depending on the operating system, etc. That's when this macro is useful. It returns a `bool`.

```rust
fn main() {
    let helpful_message = if cfg!(windows) { "backslash" } else { "slash" };

    println!(
        "...then in your hard drive, type the directory name followed by a {}. Then you...",
        helpful_message
    );
}
```

This will print differently, depending on your system. The Rust Playground runs on Linux, so it will print:

```text
...then in your hard drive, type the directory name followed by a slash. Then you...
```

`cfg!()` works for any kind of configuration. Here is an example of a function that runs differently when you use it inside a test.

```rust
#[cfg(test)] // cfg! will know to look for the word test
mod testing {
    use super::*;
    #[test]
    fn check_if_five() {
        assert_eq!(bring_number(true), 5); // This bring_number() function should return 5
    }
}

fn bring_number(should_run: bool) -> u32 { // This function takes a bool as to whether it should run
    if cfg!(test) && should_run { // if it should run and has the configuration test, return 5
        5
    } else if should_run { // if it's not a test but it should run, print something. When you run a test it ignores println! statements
        println!("Returning 5. This is not a test");
        5
    } else {
        println!("This shouldn't run, returning 0."); // otherwise return 0
        0
    }
}

fn main() {
    bring_number(true);
    bring_number(false);
}
```

Now it will run differently depending on the configuration. If you just run the program, it will give you this:

```text
Returning 5. This is not a test
This shouldn't run, returning 0.
```

But if you run it in test mode (`cargo test` for Rust on your computer), it will actually run the test. And because the test always returns 5 in this case, it will pass.

```text
running 1 test
test testing::check_if_five ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```


# Part 2 - Rust on your computer

You saw that we can learn almost anything in Rust just using the Playground. But if you learned everything so far, you will probably want Rust on your computer now. There are always some things that you can't do with the Playground like opening files or writing Rust in more than one just file. Some other things you need Rust on your computer for are input and flags. But most important is that with Rust on your computer you can use crates. We already learned about crates, but in the Playground you could only use the most popular ones. But with Rust on youn computer you can use any crate in your program.

