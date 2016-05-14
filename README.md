querystring
===========

A simple query string parser for rust that returns a HashMap of strings.

*If you want the result in a Json structure or nesting support, take a look
at this other (unrelated) project:
[queryst](https://github.com/rustless/queryst)*

# Usage

Add the dependency to Cargo.toml:

```toml
[dependencies]
querystring = { git = "https://github.com/briansteffens/querystring" }
```

Update dependencies to fetch the dependency:

```bash
cargo update
```

Here's a quick code example:

```rust
extern crate querystring;

use querystring::parse;

fn main() {
    let parsed = match parse("here=are&some=values") {
        Ok(v) => v,
        Err(e) => panic!("Parsing error: {}", e),
    };

    for (key, value) in parsed {
        println!("{} = {}", key, value);
    }
}
```
