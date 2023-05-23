# Learn Rust

## Resources

1. [Rust Book](https://doc.rust-lang.org/book/)
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
