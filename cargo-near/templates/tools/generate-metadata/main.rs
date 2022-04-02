extern crate contract;

extern "Rust" {
    fn __near_metadata() -> near_sdk::Metadata;
}

fn main() -> Result<(), std::io::Error> {
    let metadata = unsafe { __near_metadata() };
    let contents = serde_json::to_string_pretty(&metadata)?;
    print!("{}", contents);
    Ok(())
}
