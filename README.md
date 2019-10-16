> “Times are bad. Children no longer obey their parents, and everyone is
> writing an error handling library.”
> — Cicero

fast, cheap, and out of control exceptions for rust.

# Usage

First, add this to your `Cargo.toml`:
```toml
[dependencies]
eliza_error = "0.99.0";
```

You can now throw exceptions!

```rust
use eliza_error::{Error, throw};

fn my_great_function() -> Result<(), Error> {
    if is_everything_terrible() {
        throw!("everything is terrible!");
    }
    Ok(())
}
```

Eliza errors also work fine with errors from the standard library.

```rust
use eliza_error::{Error, throw};
use std::fs::File;
use std::io::prelude::*;

fn look_at_file() -> Result<(), Error> {
    let mut file = File::open("this_file_doesnt_exist.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents == "everything is terrible!!!" {
        throw!("wow, everything is still terrible!!!");
    }
    Ok(())
}
```

# Why should I use this?

- `eliza_error` has no dependencies!
- `eliza_error` can throw exceptions!
- `eliza_error` [will support backtraces soon][backtrace]!

[backtrace]: https://github.com/rust-lang/rust/issues/53487
