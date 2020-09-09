## VecDeque

A `VecDeque` is a `Vec` that is good at popping items both off the front and the back. When you use `.pop()` on a `Vec`, it just takes off the last item on the right and nothing else is moved. But if you take it off another part, all the items to the right are moved over one position to the left. You can see this in the description for `.remove()`:

```text
Removes and returns the element at position index within the vector, shifting all elements after it to the left.
```

So if you do this:

```rust
fn main() {
    let mut my_vec = vec![9, 8, 7, 6, 5];
    my_vec.remove(0);
}
```

it will remove `9`. `8` in index 1 will move to index 0, `7` in index 2 will move to index 1, and so on.

You don't have to worry about that with a `VecDeque`. It is usually a bit slower than a `Vec`, but if you have to do things on both ends then it is a better solution.

In this example we have a `Vec` of things to do. Then we make a `VecDeque` and use `.push_front()` to put them at the front, so the first item we added will be on the right. But each item we push is a `(&str, bool)`: `&str` is the description and `false` means it's not done yet. We use our `done()` function to pop an item off the back, but we don't want to delete it. Instead, we change `false` to `true` and push it at the front.

It looks like this:

```rust
use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) { // Each item is a (&str, bool)
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap(); // pop off the back
    task_done.1 = true;	                           // now it's done - mark as true
    input.push_front(task_done);                   // put it at the front now
}

fn main() {
    let mut my_vecdeque = VecDeque::new();
    let things_to_do = vec!["send email to customer", "add new product to list", "phone Loki back"];

    for thing in things_to_do {
        my_vecdeque.push_front((thing, false));
    }

    done(&mut my_vecdeque);
    done(&mut my_vecdeque);

    check_remaining(&my_vecdeque);

    for task in my_vecdeque {
        print!("{:?} ", task);
    }
}
```

This prints:

```text
You must: phone Loki back
("add new product to list", true) ("send email to customer", true) ("phone Loki back", false) 
```


