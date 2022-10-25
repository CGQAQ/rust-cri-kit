use cri_kit::build::cri_init;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cri_init("foo")?;

    Ok(())
}
