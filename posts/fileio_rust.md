---
title: File Processing in Rust
description: A simple example to showcase how to interact with files in rust.
date: 2024-01-08
image_path: '/fileio_rust.jpeg'
tags: [tech]
---

Following on from the console input post, here's the other thing I do constantly in Python and had to relearn in Rust: read a file, do something to what's in it, write the result back out.

The program below reads a text file of names and dates of birth, formats them into a table, and writes that table to a second file. Nothing clever, but it covers most of what I actually need day to day.

## Reading the file

```rust
let contents = fs::read_to_string("./DOB.txt").expect("File path does not exist");
```

`std::fs` pulls the whole file into a string in one line. `expect` panics if the file isn't there, which is fine for a small tool where you want to know immediately and loudly. Anything longer lived and you'd match on the `Result` properly.

## Parsing the contents

```rust
let vec_contents: Vec<Vec<_>> = contents
    .lines()
    .map(|line| line.split_whitespace().collect())
    .collect();
```

Each line gets split on whitespace, so you end up with a vector of vectors where the inner one holds the words of that line. Coming from Python, this is the part that felt most familiar. It's a list comprehension with more ceremony around the types.

## Formatting and printing

```rust
println!("Name | DOB\n=========================");
let mut data = String::from("Name | DOB\n=========================");
vec_contents.iter().for_each(|p| {
    println!("{} | {}", p[0..2].join(" "), p[p.len() - 3..].join(" "));
    data.push_str(&format!(
        "\n{} | {}",
        p[0..2].join(" "),
        p[p.len() - 3..].join(" ")
    ))
});
```

The assumption here is that the first two elements of each line are the name and the last three are the date of birth. They get joined with a vertical bar, printed, and appended to `data` for writing out later.

That indexing is the fragile part. `p[p.len() - 3..]` will panic on any line with fewer than three whitespace separated chunks, so the input file has to be well behaved. For something I run on a file I control, I'll take that trade.

## Writing the output

```rust
let _ = fs::write("./output.txt", data);
```

`fs::write` creates `output.txt`, or overwrites it if it already exists. The `let _ =` quietly throws away the `Result`, which is another thing I wouldn't do in anything that mattered.

Read, split, join, write. The standard library covers the whole thing without a single dependency, and that's still the bit that surprises me most coming from Python.

[Source](https://github.com/jigypeper/file-read)
