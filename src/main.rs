mod parser;
mod repl;

fn main() -> Result<(), String> {
    repl::run()?;

    Ok(())
}
