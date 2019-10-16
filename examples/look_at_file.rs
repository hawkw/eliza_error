use eliza_error::{throw, Error};
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

fn main() -> Result<(), Error> {
    look_at_file()
}
