## HashMap (and BTreeMap)

A HashMap is a collection made out of *keys* and *values*. You use the key to look up the value that matches the key. You can create a new `HashMap` with just `HashMap::new()` and use `.insert(key, value)` to insert items.

A `HashMap` is not in order, so if you print every key in a `HashMap` together it will probably print differently. We can see this in an example:

```rust
use std::collections::HashMap; // You have to bring HashMap in to use it

struct City {
    name: String,
    population: HashMap<u32, u32>, // This will have the date and the population for the date
}

fn main() {

    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(), // So far the HashMap is empty
    };

    tallinn.population.insert(1372, 3_250); // insert three dates
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);


    for (year, population) in tallinn.population { // The HashMap is HashMap<u32, u32> so it returns a tuple with two items
        println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
    }
}
```

This prints:

```text
In the year 1372 the city of Tallinn had a population of 3250.
In the year 2020 the city of Tallinn had a population of 437619.
In the year 1851 the city of Tallinn had a population of 24000.
```

or it might print:

```text
In the year 1851 the city of Tallinn had a population of 24000.
In the year 2020 the city of Tallinn had a population of 437619.
In the year 1372 the city of Tallinn had a population of 3250.
```

If you want a `HashMap` that you can sort, you can use a `BTreeMap`. Actually they are very similar to each other, so we can quickly change our `HashMap` to a `BTreeMap` to see. You will notice that it is almost the same code.

```rust
use std::collections::BTreeMap; // Just change HashMap to BTreeMap

struct City {
    name: String,
    population: BTreeMap<u32, u32>, // Just change HashMap to BTreeMap
}

fn main() {

    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(), // Just change HashMap to BTreeMap
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population.iter() { // just add .iter() - it will be sorted
        println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
    }
}
```

Now it will always print:

```text
In the year 1372 the city of Tallinn had a population of 3250.
In the year 1851 the city of Tallinn had a population of 24000.
In the year 2020 the city of Tallinn had a population of 437619.
```

Now we will go back to `HashMap`.

You can get a value in a `HashMap` by just putting the key in `[]` square brackets. This will bring up the value for the key `Bielefeld`, which is `Germany`. But be careful, because the program will crash if there is no key. If you write `println!("{:?}", city_hashmap["Bielefeldd"]);` for example then it will crash, because `Bielefeldd` doesn't exist. If you are not sure that there will be a key, you can use `.get()` which returns an `Option`. Then you will get `None` instead of crashing the program.

```rust
use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd")); 
}
```

This prints:

```text
"Germany"
Some("Germany")
None
```

because *Bielefeld* exists, but *Bielefeldd* does not exist.

If a `HashMap` already has a key when you try to put it in, it will overwrite the value that matches it:

```rust
use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();
    
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(&1)); 
}
```

This prints `Some("Eye of the World")`, because it was the last one you used `.insert()` for.

It is easy to check if an entry exists, because you can check with `.get()` which gives an `Option`:

```rust
use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");

    if book_hashmap.get(&1).is_none() {
        book_hashmap.insert(1, "Le Petit Prince");
    }

    println!("{:?}", book_hashmap.get(&1));
}
```

On this subject, `HashMap` has a very interesting method called `.entry()`. With this you can try to make an entry and use another method like `.or_insert()` to insert the value if there is no key. The interesting part is that it also gives a mutable reference so you can change it. First is an example where we just insert `true` every time we insert a book title into the `HashMap`.

```rust
use std::collections::HashMap;

fn main() {
    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"]; // Eye of the World appears twice

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
}
```

But maybe it would be better to count the number of books so that we know that there are two copies of *Eye of the World*. First let's look at what `.entry()` does, and what `.or_insert()` does. `.entry()` actually returns an `enum` called `Entry`:

```rust
pub fn entry(&mut self, key: K) -> Entry<K, V> // 🚧
```


[Here is the page for Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html). There we can see the code for it:

```rust
use std::collections::hash_map::*;

pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

fn main() { }
```

Then when we call `.or_insert()`, it looks at the enum and decides what to do.

```rust
pub fn or_insert(self, default: V) -> &'a mut V { // 🚧
    match self {
        Occupied(entry) => entry.into_mut(),
        Vacant(entry) => entry.insert(default),
    }
}
```

The interesting part is that it returns a `mut` reference. That means you can bind it to a variable, and change the variable to change the value in the `HashMap`. So for every book we will insert a 0 if there is no entry, and we will use `+= 1` on the reference. Now it looks like this:

```rust
use std::collections::HashMap;

fn main() {
    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        *return_value +=1;
    }

    for (book, number) in book_hashmap {
        println!("{:?}, {:?}", book, number);
    }
}
```

The important part is `let return_value = book_hashmap.entry(book).or_insert(0);`. If you take out the variable, you get `book_hashmap.entry(book).or_insert(0)`. If you do that then the mutable reference just doesn't go anywhere: it inserts 0, and then has a mutable reference to 0 that nobody takes. So we bind it to `return_value` so we can keep the 0. Then we increase the value by 1, which gives at least 1 for every book in the `HashMap`. Then when `.entry()` looks at *Eye of the World* again it doesn't insert anything, but it gives us a mutable 1. Then we increase it to 2, and that's why it prints this:

```text
"L\'Allemagne Moderne", 1
"Eye of the World", 2
"Le Petit Prince", 1
```

You can also use `.or_insert_with()` which lets you use a closure. You can always just do this:

```rust
// 🚧
let return_value = book_hashmap.entry(book).or_insert_with(|| 0); // Closure with nothing
```

or add any logic that you want. Here is something simple.

```rust
    // 🚧
    for book in book_collection {
        let return_value =
            book_hashmap.entry(book).or_insert_with(|| {
                    if book == "Eye of the World" { // Maybe an extra copy is arriving next week
                                                    // so we know there will be at least one more
                        1
                    } else {
                        0
                    }
                },
            );
        *return_value += 1;
    }
```

You can also do things with `.or_insert()` like insert a vec and then push into the vec. Let's pretend that we asked men and women on the street what they think of a politican. They give a rating from 0 to 10. Then we want to put the numbers together to see if the politician is more popular with men or women. It can look like this:


```rust
use std::collections::HashMap;

fn main() {
    let data = [ // This is the raw data
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for item in data.iter() { // This gives a tuple of &(&str, i32)
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    for (male_or_female, numbers) in survey_hash {
        println!("{:?}: {:?}", male_or_female, numbers);
    }
}
```

This prints:

```text
"female", [5, 6, 5]
"male", [9, 0, 10]
```

The important line is: `survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);` So if it sees "female" it will check to see if there is "female" already in the `HashMap`. If not, it will insert a `Vec::new()`, then push the number in. If it sees "female" already in the `HashMap`, it will not insert a new Vec, and will just push the number into it.

