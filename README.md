# Learn Rust

## Resources

1. [Rust Book](https://rust-book.cs.brown.edu/)
2. [Rust Book - Interactive version](https://rust-book.cs.brown.edu/)
3. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
4. [Set up Rust in VSCode](https://users.rust-lang.org/t/setting-up-rust-with-vs-code/76907)
5. [Learn Rust in 2020](https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md)

## Setting up Rust and VSCode for learning and development

1. Install Rust: Follow the instruction in the book
2. Set up VSCode:

  - Install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
	
	- Disable Inlay Hints: `rust-analyzer.inlayHints.enable: false`

  - Install the [crates extension](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)

	- Update *Local Cargo Index Branch* to `origin/HEAD`: `crates.localCargoIndexBranch: "origin/HEAD"`

  - Install the [CodeLLDB extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
  - Install the [Code Runner extension](https://marketplace.visualstudio.com/items?itemName=formulahendry.code-runner)
	
	- Update *Code-runner: Executor Map*: `"rust": "cargo run # $fileName",`
	- Enable *File Directory as Cwd*: `"code-runner.fileDirectoryAsCwd": true,`

  - Install the [Better TOML extension](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)

## The Rust Book - Progress

- [X] [Chapter 1 - Getting Started](https://rust-book.cs.brown.edu/ch01-00-getting-started.html)
- [X] [Chapter 2 - Programming a Guessing Game](https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html)
- [ ] [Chapter 3 - Common Programming Concepts](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)
- [ ] [Chapter 4 - Understanding Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html)
- [ ] [Chapter 5 - Using Structs to Structure Related Data](https://rust-book.cs.brown.edu/ch05-00-structs.html)
- [ ] [Chapter 6 - Enums and Pattern Matching](https://rust-book.cs.brown.edu/ch06-00-enums.html)
- [ ] [Chapter 7 - Managing Growing Projects with Packages, Crates, and Modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [ ] [Chapter 8 - Common Collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html)
- [ ] [Chapter 9 - Error Handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html)
- [ ] [Chapter 10 - Generic Types, Traits, and Lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html)
- [ ] [Chapter 11 - Writing Automated Tests](https://rust-book.cs.brown.edu/ch11-00-testing.html)
- [ ] [Chapter 12 - An I/O Project: Building a Command Line Program](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html)
- [ ] [Chapter 13 - Functional Language Features: Iterators and Closures](https://rust-book.cs.brown.edu/ch13-00-functional-features.html)
- [ ] [Chapter 14 - More about Cargo and Crates.io](https://rust-book.cs.brown.edu/ch14-00-more-about-cargo.html)
- [ ] [Chapter 15 - Smart Pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html)
- [ ] [Chapter 16 - Fearless Concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html)
- [ ] [Chapter 17 - Object Oriented Programming Features of Rust](https://rust-book.cs.brown.edu/ch17-00-oop.html)
- [ ] [Chapter 18 - Patterns and Matching](https://rust-book.cs.brown.edu/ch18-00-patterns.html)
- [ ] [Chapter 19 - Advanced Features](https://rust-book.cs.brown.edu/ch19-00-advanced-features.html)
- [ ] [Chapter 20 - Final Project: Building a Multithreaded Web Server](https://rust-book.cs.brown.edu/ch20-00-final-project-a-web-server.html)
