
use std::fs;


fn scrapy_url() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_scrapy_url() {
        scrapy_url().unwrap();
    }

}