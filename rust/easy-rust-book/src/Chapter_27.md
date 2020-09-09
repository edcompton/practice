## Implementing structs and enums

To call functions on a `struct` or an `enum`, use an `impl` block. These functions are called **methods**. There are two kinds of methods in an `impl` block.

- Regular methods: these take **self** (or **&self** or **&mut self**). Regular methods use a `.`. `.clone()` is a regular method.
- Associated methods (or "static" methods): these do not take self. They are written differently, using `::`. `String::from()` is an associated method. You usually use associated methods to create new variables.

In our example we are going to create animals and print them. For a new struct or enum, you need to give it **Debug** if you want to use `{:?}` to print. If you write `#[derive(Debug)]` above the struct or enum then you can print it with `{:?}`. These messages with `#[]` are called **attributes**. You can sometimes use them to tell the compiler to give your struct an ability like `Debug`. There are many attributes and we will learn about them later. But derive is probably the most common and you see it a lot above structs and enums.

```rust
#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new() -> Self {
        // Self means Animal.
        //You can also write Animal instead of Self

        Self {
            // When we write Animal::new(), we always get a cat that is 10 years old
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        // use .change_to_dog() to change the cat to a dog
        // with &mut self we can change it
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        // use .change_to_cat() to change the dog to a cat
        // with &mut self we can change it
        println!("Changing animal to cat!");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        // we want to read self
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

fn main() {
    let mut new_animal = Animal::new(); // Associated method to create a new animal
                                        // It is a cat, 10 years old
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
}
```

This prints:

```text
The animal is a cat
Changing animal to dog!
The animal is a dog
Changing animal to cat!
The animal is a cat
```

### Self

Remember that Self (the type Self) and self (the variable self) are abbreviations. (abbreviation = short way to write)

So in our code, Self = Animal. Also, `fn change_to_dog(&mut self)` means `fn change_to_dog(&mut Animal)`

