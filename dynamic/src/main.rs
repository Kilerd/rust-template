use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

use regex::{Regex};

fn main() -> Result<(), Error> {
    let mut parameters = HashMap::new();
    parameters.insert(format!("a"), format!("b"));
    let result = render("test.txt", &parameters)?;
    println!("{}", result);

    Ok(())
}


fn render(template_name: &'static str, parameters: &HashMap<String, String>) -> Result<String, Error> {
    // loading file content
    let mut result = File::open(template_name)?;
    let mut content = String::new();
    result.read_to_string(&mut content)?;

    // template engine
    let parameter_pattern = Regex::new(r"\{\{\s?(?P<key>[a-zA-Z_]+)\s?}}").unwrap();
    let result = parameter_pattern.replace_all(&content, |cap: &regex::Captures| {
        dbg!(cap);
        if let Some(param) = cap.name("key") {
            parameters.get(param.as_str()).unwrap_or(&cap.get(0).unwrap().as_str().to_string()).as_str().to_string()
        } else {
            format!("")
        }
    });
    Ok(result.to_string())
}