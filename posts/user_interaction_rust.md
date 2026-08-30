---
title: Building Console Apps in Rust
description: A Python Developer's Perspective
date: 2023-12-23
image_path: '/io_rust_blog.jpeg'
tags: [tech]
---

As a software engineer manily working with Python, I've recently been diving deeper into Rust, exploring how it handles common programming tasks that I usually breeze through in Python. Today, I want to share my experience building console applications in Rust, specifically focusing on user input handling and how it differs from Python's approach.

## The Journey from Python to Rust

Those of you familiar with Python know how straightforward handling console input can be. A simple call to `input()` and you're done:

```python
name = input("what is your name? ")
print(f"Hello {name}!")
```

When I first approached the same task in Rust, I was struck by how much more explicit we need to be. Here's the equivalent code:

```rust
use std::io;

fn main() {
    println!("What is your name?");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    println!("Hello {}!", input.trim());
}
```

At first glance, this might seem like unnecessary complexity. However, working with Rust has helped me appreciate why this verbosity exists. The explicit handling of mutable strings and error cases forces us to think about aspects of our program that Python abstracts away.

## Creating Better Abstractions

While I appreciate Rust's explicit approach, there's nothing wrong with creating our own abstractions once we understand what's happening under the hood. Here's a utility function I've been using that provides a more familiar interface while maintaining Rust's safety guarantees:

```rust
fn ask_question(question: &str) -> String {
    println!("{question}");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Looks like there be a sea monster in the I/O waters.");

    input.trim().to_string()
}
```

This gives us a more concise way to handle user input while still leveraging Rust's safety features. Here's how we can use it in a complete program:

```rust
use std::io;

fn ask_question(question: &str) -> String {
    println!("{question}");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Looks like there be a sea monster in the I/O waters.");

    input.trim().to_string()
}

fn main() {
    println!("🌟 Welcome to the Rustic Adventure! 🚀");

    let name = ask_question("What do they call ye, brave coder?");
    let age = ask_question("How many orbits around the sun have ye completed?");

    println!("🎉 Ahoy, {name}! Ready to set sail into the Rustic seas\nof coding at the youthful age of {age}? 🚢⚓");
}
```

## Reflections on the Rust Approach

Coming from Python, Rust's approach to console I/O initially felt overly complex. However, this complexity serves a purpose. By making things explicit, Rust encourages us to think about:

- Memory management and string mutability
- Error handling and edge cases
- Creating clean abstractions while maintaining safety

I still reach for Python when I want to prototype something quickly, but for anything where reliability and explicit error handling matter, Rust forcing the issue has grown on me.

Whether you want the verbose version or something like `ask_question` mostly comes down to how many times you're going to call it. Two or three and I'd leave it inline. Any more than that and the helper earns its place.

Ahmed


[Source](https://github.com/jigypeper/user-interaction)
