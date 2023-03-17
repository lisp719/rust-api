use include_dir::{include_dir, Dir};
use serde_json::json;
use tera::{Context, Tera};

const TEMPLATES: Dir = include_dir!("templates");

fn main() {
    let mut tera = Tera::default();
    for file in TEMPLATES.files() {
        tera.add_raw_template(file.path().to_str().unwrap(), file.contents_utf8().unwrap())
            .expect("Could not add template");
    }

    let data = json!({"name": "John Doe"});
    let context = Context::from_serialize(data).expect("Could not serialize");

    match tera.render("hi.txt", &context) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }

    match tera.render("bye.txt", &context) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }
}
