use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    vim_build::configurator("src/vim".to_string())
        .generate()?;
    Ok(())
}