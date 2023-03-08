use handlebars::Handlebars;
use rust_embed::RustEmbed;
use std::{collections::BTreeMap, io::stdin};

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars.register_embed_templates::<Asset>().unwrap();

    let mut data = BTreeMap::new();
    let mut name = String::new();

    println!("Please enter your name");
    stdin().read_line(&mut name).unwrap();
    data.insert("name", name.trim_end().to_string());

    match handlebars.render("hi.txt", &data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }

    match handlebars.render("bye.txt", &data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }
}
