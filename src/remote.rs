use std::error::Error;

pub fn sync() -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get("http://127.0.0.1:3000/")?.text()?;
    println!("{:#?}", response);
    Ok(())
}
