use eliza_error::{throw, Error};

fn is_everything_terrible() -> bool {
    true
}

fn my_great_function() -> Result<(), Error> {
    if is_everything_terrible() {
        throw!("everything is terrible!");
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    my_great_function()
}
